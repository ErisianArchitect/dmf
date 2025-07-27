
pub trait Replace: Sized {
    fn replace(&mut self, src: Self) -> Self;
    fn replace_with<F: FnOnce(Self) -> Self>(&mut self, replace: F);
}

impl<T: Sized> Replace for T {
    fn replace(&mut self, src: Self) -> Self {
        std::mem::replace(self, src)
    }

    fn replace_with<F: FnOnce(Self) -> Self>(&mut self, replace: F) {
        unsafe {
            std::ptr::write(self, replace(std::ptr::read(self)));
        }
    }
}