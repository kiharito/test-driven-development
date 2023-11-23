#[derive(Debug, PartialEq)]
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
}

#[derive(Debug, PartialEq)]
pub struct Franc {
    amount: u32,
}

impl Franc {
    pub fn new(amount: u32) -> Self {
        Franc { amount }
    }
    pub fn times(&self, multiplier: u32) -> Franc {
        Franc::new(self.amount * multiplier)
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
        assert!(Dollar::new(5).eq(&Dollar::new(5)));
        assert!(!Dollar::new(5).eq(&Dollar::new(6)))
    }

    #[test]
    fn test_franc_multiplication() {
        let five = Franc::new(5);
        assert_eq!(Franc::new(10), five.times(2));
        assert_eq!(Franc::new(15), five.times(3));
    }
}
