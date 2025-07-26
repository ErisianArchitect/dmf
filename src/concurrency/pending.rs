use std::{
    alloc::{
        alloc, dealloc, Layout
    }, cell::UnsafeCell, mem::{
        MaybeUninit,
    }, ptr::NonNull, sync::atomic::{AtomicU8, Ordering}
};
use super::error::*;

const TAKEN: u8 = 0;
const WAITING: u8 = 1;
const ASSIGNING: u8 = 2;
const READY: u8 = 4;

// pub trait SpawnStrategy: crate::Sealed<Pending<()>> {
//     fn spawn<F: FnOnce() + Send + 'static>(f: F);
// }

// pub struct StdThread;
// pub struct RayonThread;

// impl crate::Sealed<Pending<()>> for StdThread {}
// impl crate::Sealed<Pending<()>> for RayonThread {}
// impl SpawnStrategy for StdThread {
//     fn spawn<F: FnOnce() + Send + 'static>(f: F) {
//         std::thread::spawn(f);
//     }
// }
// impl SpawnStrategy for RayonThread {
//     fn spawn<F: FnOnce() + Send + 'static>(f: F) {
//         rayon::spawn(f);
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum PendingError {
    #[error("Value already taken.")]
    Taken,
    #[error("Assigning value.")]
    Assigning,
    #[error("Waiting for value.")]
    Waiting,
    #[error("Unknown state: {0}")]
    UnknownState(u8),
}

struct Inner<R> {
    // we only need AtomicU8 since there can only be one sender and one receiver.
    ref_count: AtomicU8,
    state: AtomicU8,
    result: UnsafeCell<MaybeUninit<R>>,
}

impl<R> Inner<R> {
    const fn layout() -> Layout {
        Layout::new::<Self>()
    }

    unsafe fn alloc_new() -> Result<NonNull<Inner<R>>> {
        unsafe {
            let layout = Self::layout();
            let ptr = alloc(layout) as *mut Self;
            let raw = NonNull::new(ptr).ok_or_else(|| Error::OutOfMemory)?;
            raw.write(Self {
                // initial reference count of 2 because there is one sender and one receiver.
                ref_count: AtomicU8::new(2),
                state: AtomicU8::new(WAITING),
                result: UnsafeCell::new(MaybeUninit::uninit()),
            });
            Ok(raw)
        }
    }

    /// Decrements the reference count and drops then deallocs if the reference count becomes 0.
    unsafe fn decrement_ref_count(raw: NonNull<Self>) -> bool {
        let inner_ref = unsafe { raw.as_ref() };
        if inner_ref.ref_count.fetch_sub(1, Ordering::AcqRel) == 1 {
            unsafe { Self::drop_and_dealloc(raw); }
            true
        } else {
            false
        }
    }

    /// This function should only ever be called on the last instance.
    unsafe fn drop_and_dealloc(raw: NonNull<Self>) {
        let mut raw = raw;
        unsafe  {
            let inner_mut = raw.as_mut();
            let state = inner_mut.state.load(Ordering::Acquire);
            match state {
                TAKEN | WAITING => (/* Do nothing, there is no value. */),
                ASSIGNING => {
                    unreachable!("Invalid state on cleanup.");
                }
                READY => {
                    inner_mut.result.get_mut().assume_init_drop();
                }
                unknown => unreachable!("Unknown state: {unknown}"),
            }
            dealloc(raw.as_ptr() as *mut _, Self::layout());
        }

    }
}

pub struct Pending<R: Send + 'static> {
    raw: NonNull<Inner<R>>,
}

pub struct Responder<R: Send + 'static> {
    raw: NonNull<Inner<R>>,
}

unsafe impl<R> Send for Responder<R>
where R: Send + 'static {}
unsafe impl<R> Sync for Responder<R>
where R: Send + Sync + 'static {}

impl<R: Send + 'static> Responder<R> {
    #[must_use]
    #[inline]
    fn from_raw(raw: NonNull<Inner<R>>) -> Self {
        Self { raw }
    }

    #[inline(always)]
    pub fn respond(self, result: R) {
        unsafe {
            let inner_ref = self.raw.as_ref();
            // We use store because this is the only thing that can modify the value.
            // The responder is the writer, and the `Pending` is the reader.
            inner_ref.state.store(ASSIGNING, Ordering::Release);
            inner_ref.result.get().write(MaybeUninit::new(result));
            inner_ref.state.store(READY, Ordering::Release);
        }
    }
}

impl<R: Send + 'static> Pending<R> {

    #[must_use]
    #[inline]
    fn from_raw(raw: NonNull<Inner<R>>) -> Self {
        Self { raw }
    }

    #[must_use]
    #[inline]
    pub fn pair() -> (Self, Responder<R>) {
        let raw = unsafe {
            Inner::<R>::alloc_new().expect("Out of memory.")
        };
        (
            Self::from_raw(raw),
            Responder::from_raw(raw)
        )
    }

    #[must_use]
    #[inline]
    pub fn spawn<F: FnOnce() -> R + Send + 'static>(worker: F) -> Self {
        let (pending, responder) = Self::pair();
        rayon::spawn(move || {
            responder.respond(worker());
        });
        pending
    }

    #[inline]
    pub fn is_ready(&self) -> bool {
        let inner_ref = unsafe {
            self.raw.as_ref()
        };
        inner_ref.state.compare_exchange(READY, READY, Ordering::AcqRel, Ordering::Relaxed).is_ok()
    }

    #[must_use]
    #[inline]
    pub fn try_recv(&self) -> std::result::Result<R, PendingError> {
        unsafe {
            let inner_ref = self.raw.as_ref();
            match inner_ref.state.compare_exchange(READY, TAKEN, Ordering::AcqRel, Ordering::Relaxed) {
                Ok(_) => Ok(inner_ref.result.get().read().assume_init()),
                Err(TAKEN) => Err(PendingError::Taken),
                Err(WAITING) => Err(PendingError::Waiting),
                Err(ASSIGNING) => Err(PendingError::Assigning),
                Err(unknown) => Err(PendingError::UnknownState(unknown)),
            }
        }
    }
}

impl<R: Send + 'static> Drop for Pending<R> {
    fn drop(&mut self) {
        unsafe {
            Inner::<R>::decrement_ref_count(self.raw);
        }
    }
}

impl<R: Send + 'static> Drop for Responder<R> {
    fn drop(&mut self) {
        unsafe {
            Inner::<R>::decrement_ref_count(self.raw);
        }
    }
}

#[must_use]
#[inline]
pub fn pair<R: Send + 'static>() -> (Pending<R>, Responder<R>) {
    Pending::pair()
}

#[must_use]
#[inline]
pub fn spawn<R: Send + 'static, F: FnOnce() -> R + Send + 'static>(worker: F) -> Pending<R> {
    Pending::spawn(worker)
}