struct Solution;

use std::collections::BTreeSet;

impl Solution {
    fn cleanup_email(email: &str) -> String {
        let mut out = String::new();
        out.reserve(email.len());
        // we will do this in one scan only!!
        let mut iter = email.chars();
        // stage 1: local address
        let mut skipping = false;
        for ch in iter.by_ref() {
            if ch == '@' { break }
            else if ch == '+' { skipping = true; }
            else if ch != '.' && !skipping { out.push(ch); }
        }
        out.push('@');
        // stage 2: domain name
        out + iter.as_str()
    }

    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        emails.iter()
            .map(|email| Solution::cleanup_email(email))
            .collect::<BTreeSet<String>>()
            .len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cleanup_email() {
        let cleaned_up = Solution::cleanup_email("michael.peng+piazza@umich.edu");
        assert_eq!(cleaned_up, "michaelpeng@umich.edu");
    }

    #[test]
    fn unique_emails() {
        let emails: Vec<String> = vec![
            "abc.def+gghh@mail.com",
            "ab...c.def@mail.com",
            "a..bc.de..f@ma.il.com",
            "abcdef@mail.com",
            "abcde+f@mail.com"
        ].into_iter().map(|c| c.to_owned()).collect();
        assert_eq!(Solution::num_unique_emails(emails), 3);
    }
}