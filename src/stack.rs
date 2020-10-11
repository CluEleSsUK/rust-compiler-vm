pub trait Stack<'a, T: Clone> {
    fn pop(&mut self) -> Option<Box<T>>;
    fn peek(&self) -> Option<Box<T>>;
    fn push(&mut self, item: Box<T>) -> &mut dyn Stack<T>;
}

pub struct VectorStack<T> {
    items: Vec<Box<T>>
}

impl<T> VectorStack<T> {
    pub const fn new() -> VectorStack<T> {
        return VectorStack {
            items: Vec::new()
        };
    }
}

impl<'a, T: Clone> Stack<'a, T> for VectorStack<T> {
    fn pop(&mut self) -> Option<Box<T>> {
        return if self.items.is_empty() {
            None
        } else {
            let last = self.items.last().cloned();
            self.items = self.items[0..(self.items.len() - 1)].to_vec();
            last
        };
    }

    fn peek(&self) -> Option<Box<T>> {
        return match self.items.get(self.items.len() - 1) {
            None => None,
            Some(x) => Some(x.clone())
        };
    }

    fn push(&mut self, item: Box<T>) -> &mut dyn Stack<T> {
        self.items.push(item);
        return self;
    }
}

