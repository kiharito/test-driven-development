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

#[derive(Debug)]
pub struct Franc {
    amount: u32,
}

impl PartialEq for Franc {
    fn eq(&self, franc: &Self) -> bool {
        self.amount == franc.amount()
    }
}

impl Franc {
    pub fn new(amount: u32) -> Self {
        Franc { amount }
    }
    pub fn times(&self, multiplier: u32) -> Franc {
        Franc::new(self.amount * multiplier)
    }
    pub fn amount(&self) -> u32 {
        self.amount
    }
    pub fn equals(&self, franc: Self) -> bool {
        self.eq(&franc)
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

    #[test]
    fn test_franc_multiplication() {
        let five = Franc::new(5);
        assert_eq!(Franc::new(10), five.times(2));
        assert_eq!(Franc::new(15), five.times(3));
    }
}
