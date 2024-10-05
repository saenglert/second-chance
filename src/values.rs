#[derive(Debug)]
pub struct ISBN {
    value: u64
}

impl ISBN {
    const MIN_10_DIGITS: u64 = 1000000000;
    const MAX_10_DIGITS: u64 = 9999999999;

    const MIN_13_DIGITS: u64 = 1000000000000;
    const MAX_13_DIGITS: u64 = 9999999999999;

    pub fn new(value: u64) -> Result<Self, String> {
        if 
            value > ISBN::MAX_13_DIGITS
            || value < ISBN::MIN_10_DIGITS
            || (value > ISBN::MAX_10_DIGITS && value < ISBN::MIN_13_DIGITS)
        {
            return Err("Invalid ISBN".to_string());
        }

        Ok(Self { value })
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}