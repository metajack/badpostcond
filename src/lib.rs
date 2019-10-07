use mirai_annotations::{postcondition, assumed_postcondition};

pub struct Foo {
    capacity: usize,
    length: usize,
}

impl Foo {
    pub fn with_capacity(capacity: usize) -> Foo {
        let result = Foo {
            capacity,
            length: 0,
        };
        postcondition!(result.length <= result.capacity);
        result
    }
    
    pub fn reserve(&mut self, additional: usize) {
        let (&mut len, cap) = self.double_mut();
        let new_cap = if cap - len < additional {
            len.checked_add(additional)
                .and_then(usize::checked_next_power_of_two)
                .unwrap_or(usize::max_value())
        } else {
            cap
        };
        postcondition!(new_cap >= cap);
    }

    fn double_mut(&mut self) -> (&mut usize, usize) {
        let result = (&mut self.length, self.capacity);
        assumed_postcondition!(*result.0 <= result.1);
        result
    }
}
