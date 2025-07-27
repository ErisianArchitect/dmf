
pub struct SealType<T>(std::marker::PhantomData<fn(T)>);

pub trait Sealed<Marker> {}