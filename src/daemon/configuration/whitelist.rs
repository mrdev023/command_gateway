use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Whitelist(Vec<String>);

impl PartialEq<&str> for Whitelist {
    fn eq(&self, other: &&str) -> bool {
        self.0.iter().any(|w| {
            match Regex::new(w) {
                Ok(regex) => regex.is_match(other),
                Err(_) => w == other
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_return_true_if_exact_value() {
        let w = Whitelist(vec!["/usr/bin/bash".into()]);
        assert_eq!(w, "/usr/bin/bash");
    }

    #[test]
    fn test_should_return_false_if_no_exact_value() {
        let w = Whitelist(vec!["/usr/bin/bash".into()]);
        assert_ne!(w, "/bin/bash");
    }

    #[test]
    fn test_should_return_true_if_regex_match() {
        let w = Whitelist(vec![r".*?_allow$".into()]);
        assert_eq!(w, "/usr/bin/bash_allow");
    }

    #[test]
    fn test_should_return_false_if_regex_not_match() {
        let w = Whitelist(vec![r".*?_allow$".into()]);
        assert_ne!(w, "/usr/bin/bash_not_allowed");
    }
}
