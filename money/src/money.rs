#[derive(Debug, PartialEq)]
struct Money {
    amount: u32,
    currency: &'static str,
}

impl Money {
    fn new(amount: u32, currency: &'static str) -> Self {
        Money { amount, currency }
    }
    fn dollar(amount: u32) -> Self {
        Money::new(amount, "USD")
    }
    fn franc(amount: u32) -> Self {
        Money::new(amount, "CHF")
    }
    fn times(&self, multiplier: u32) -> Self {
        Money::new(self.amount * multiplier, self.currency)
    }
    fn currency(&self) -> &str {
        self.currency
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
        assert!(!Money::franc(5).eq(&Money::dollar(5)));
    }

    #[test]
    fn test_franc_multiplication() {
        let five = Money::franc(5);
        assert_eq!(Money::franc(10), five.times(2));
        assert_eq!(Money::franc(15), five.times(3));
    }

    #[test]
    fn test_currency() {
        assert_eq!("USD", Money::dollar(1).currency());
        assert_eq!("CHF", Money::franc(1).currency());
    }
}
