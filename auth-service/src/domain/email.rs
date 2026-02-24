use validator::ValidateEmail;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Email(String);

impl Email {
    pub fn parse(s: String) -> Result<Email, String> {

        // .validate_email() automaticall does
        // - checks if empty
        // - check if @ is included
        // - a bunch of character length checks
        if s.validate_email() {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid email.", s))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Email;

    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use quickcheck::Gen;
    use rand::SeedableRng;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        let parsed_email = Email::parse(email);
        assert!(parsed_email.is_err());
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "someemail.com".to_string();
        let parsed_email = Email::parse(email);
        assert!(parsed_email.is_err())
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@me.com".to_string();
        let parsed_email = Email::parse(email);
        assert!(parsed_email.is_err())
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut Gen) -> Self {
            let seed = g.size() as u64;
            let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
            let email = SafeEmail().fake_with_rng(&mut rng);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        Email::parse(valid_email.0).is_ok()
    }
}