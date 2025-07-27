// A trigger is used to activate some condition from elsewhere in the program.

use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

/// One way activation trigger.
#[derive(Debug, Clone)]
pub struct Trigger {
    trigger: Arc<AtomicBool>,
}

impl Trigger {
    #[inline]
    pub fn new() -> Self {
        Self { trigger: Arc::new(AtomicBool::new(false)) }
    }

    #[inline]
    pub fn activate(&self) -> bool {
        !self.trigger.swap(true, Ordering::AcqRel)
    }

    #[inline]
    pub fn activated(&self) -> bool {
        self.trigger.load(Ordering::Acquire)
    }

    #[inline]
    pub fn trigger_ref(&self) -> TriggerRef<'_> {
        TriggerRef::new(&self.trigger)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TriggerRef<'a> {
    trigger_ref: &'a AtomicBool,
}

impl<'a> TriggerRef<'a> {
    #[inline]
    pub fn new(trigger_ref: &'a AtomicBool) -> Self {
        Self { trigger_ref }
    }

    #[inline]
    pub fn activate(self) -> bool {
        !self.trigger_ref.swap(true, Ordering::AcqRel)
    }

    #[inline]
    pub fn activated(self) -> bool {
        self.trigger_ref.load(Ordering::Acquire)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn trigger_tests() {
        fn activate_trigger_owned(trigger: Trigger) -> bool {
            trigger.activate()
        }
        fn activate_trigger_ref(trigger: TriggerRef)  -> bool {
            trigger.activate()
        }

        let trig1 = Trigger::new();
        let trig1_clone1 = trig1.clone();
        let trig1_clone2 = trig1.clone();
        assert!(!trig1.activated());
        assert!(!trig1_clone1.activated());
        assert!(!trig1_clone2.activated());
        assert!(activate_trigger_owned(trig1_clone1));
        assert!(!activate_trigger_owned(trig1_clone2));
        assert!(trig1.activated());
        let trig2 = Trigger::new();
        let trig2_ref = trig2.trigger_ref();
        assert!(!trig2.activated());
        assert!(!trig2_ref.activated());
        assert!(activate_trigger_ref(trig2_ref));
        assert!(!activate_trigger_ref(trig2_ref));
        assert!(trig2.activated());
        assert!(trig2_ref.activated());
    }
}