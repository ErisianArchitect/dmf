
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lazy<T: 'static, F: FnOnce() -> T + 'static> {
    inner: F,
}

impl<T, F: FnOnce() -> T + 'static> Lazy<T, F> {
    pub const fn new(evaluator: F) -> Self {
        Self {
            inner: evaluator,
        }
    }
    
    pub fn eval(self) -> T {
        (self.inner)()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn eval_test() {
        let lazy = Lazy::new(|| {
            let mut items = Vec::with_capacity(1024);
            for i in 0..1024 {
                items.push(String::from(format!("Lazy {i}")));
            }
            items
        });
        
        let items = lazy.eval();
        
        assert!(items.len() == 1024 && items[0] == "Lazy 0");
    }
}