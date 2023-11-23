#[derive(Debug, PartialEq)]
struct Money {
    amount: u32,
}

impl Money {
    fn new(amount: u32) -> Self {
        Money { amount }
    }
    fn dollar(amount: u32) -> Dollar {
        Dollar::new(amount)
    }
    fn franc(amount: u32) -> Franc {
        Franc::new(amount)
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
        let five = Money::dollar(5);
        assert_eq!(Money::dollar(10), five.times(2));
        assert_eq!(Money::dollar(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert!(Money::dollar(5).eq(&Money::dollar(5)));
        assert!(!Money::dollar(5).eq(&Money::dollar(6)));
        assert!(Money::franc(5).eq(&Money::franc(5)));
        assert!(!Money::franc(5).eq(&Money::franc(6)));
    }

    #[test]
    fn test_franc_multiplication() {
        let five = Money::franc(5);
        assert_eq!(Money::franc(10), five.times(2));
        assert_eq!(Money::franc(15), five.times(3));
    }
}
