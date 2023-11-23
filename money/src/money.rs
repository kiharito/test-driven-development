#[derive(Debug, PartialEq)]
struct Money {
    amount: u32,
}

impl Money {
    fn new(amount: u32) -> Self {
        Money { amount }
    }
}

#[derive(Debug, PartialEq)]
struct Dollar {
    money: Money,
}

impl Dollar {
    fn new(amount: u32) -> Self {
        Dollar {
            money: Money::new(amount),
        }
    }
    fn times(&self, multiplier: u32) -> Dollar {
        Dollar::new(self.money.amount * multiplier)
    }
}

#[derive(Debug, PartialEq)]
struct Franc {
    money: Money,
}

impl Franc {
    fn new(amount: u32) -> Self {
        Franc {
            money: Money::new(amount),
        }
    }
    fn times(&self, multiplier: u32) -> Franc {
        Franc::new(self.money.amount * multiplier)
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
        assert!(!Dollar::new(5).eq(&Dollar::new(6)));
        assert!(Franc::new(5).eq(&Franc::new(5)));
        assert!(!Franc::new(5).eq(&Franc::new(6)));
    }

    #[test]
    fn test_franc_multiplication() {
        let five = Franc::new(5);
        assert_eq!(Franc::new(10), five.times(2));
        assert_eq!(Franc::new(15), five.times(3));
    }
}
