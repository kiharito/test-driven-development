pub struct Dollar {
    amount: u32,
}

impl Dollar {
    pub fn new(amount: u32) -> Self {
        Dollar { amount }
    }
    pub fn times(&self, multiplier: u32) -> Dollar {
        Dollar::new(self.amount * multiplier)
    }
    pub fn amount(&self) -> u32 {
        self.amount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        let product = five.times(2);
        assert_eq!(10, product.amount());
        let product = five.times(3);
        assert_eq!(15, product.amount());
    }
}