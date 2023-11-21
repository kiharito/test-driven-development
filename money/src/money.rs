#[derive(Debug)]
pub struct Dollar {
    amount: u32,
}

impl PartialEq for Dollar {
    fn eq(&self, dollar: &Self) -> bool {
        self.amount == dollar.amount()
    }
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
    pub fn equals(&self, dollar: Self) -> bool {
        self.eq(&dollar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        assert_eq!(Dollar::new(10), five.times(2));
        assert_eq!(Dollar::new(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert!(Dollar::new(5).equals(Dollar::new(5)));
        assert!(!Dollar::new(5).equals(Dollar::new(6)))
    }
}
