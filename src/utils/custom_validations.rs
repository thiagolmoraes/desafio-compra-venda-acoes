use regex::Regex;
use zxcvbn::zxcvbn;

pub struct Validation {
    pub field: String
}

impl Validation {
    pub fn is_valid_email(&self) -> bool {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        email_regex.is_match(&self.field)
    }

    pub fn is_valid_password(&self) -> bool {
        let estimate = zxcvbn(&self.field, &[]);
        let score_numeric: u8 = estimate.score().into();

        score_numeric >= 3
    }
}