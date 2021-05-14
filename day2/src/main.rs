use std::ops::RangeInclusive;

#[cfg(test)]
mod tests {
    use super::PasswordPolicy;

    #[test]
    fn test_is_valid() {
        let pp = PasswordPolicy {
            byte: b'a',
            range: 1..=3,
        };

        assert_eq!(pp.is_valid("zeus"), false);
        assert_eq!(pp.is_valid("abcd"), true);
        assert_eq!(pp.is_valid("aaaa"), false);
    }

    use super::parse_line;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("1-3 a: banana").unwrap(),
            (
                PasswordPolicy {
                byte: b'a',
                range: 1..=3
            },
             "banana")
        );
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseError {
    #[error("expected {0}")]
    Expected(&'static str),
}

struct PasswordPolicy {
    byte: u8,
    range: RangeInclusive<usize>
}

impl PasswordPolicy {
    fn is_valid(&self, password: &str) -> bool {
        self.range.contains(
            &password.as_bytes()
            .iter()
            .copied()
            .filter(|&b| b == self.byte)
            .count()
        )
    }
}

use std::fmt::Debug;

impl PartialEq for PasswordPolicy {
    fn eq(&self, other: &Self) -> bool {
        self.byte == other.byte && self.range == other.range
    }
}

impl Debug for PasswordPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PasswordPolicy")
        .field("byte", &self.byte)
        .field("range", &self.range)
        .finish()
    }
}

fn parse_line(line: &str) -> anyhow::Result<(PasswordPolicy, &str)> {
    let (policy, password) = {
        let mut tokens = line.split(':');
        (
            tokens
            .next()
            .ok_or(ParseError::Expected("passowrd policy"))?,
            tokens
            .next()
            .ok_or(ParseError::Expected("password"))?
            .trim()
        )
    };

    let (range, byte) = {
        let mut tokens = policy.split(' ');
        (
            tokens.next().ok_or(ParseError::Expected("policy range"))?,
            tokens.next().ok_or(ParseError::Expected("policy byte"))?,
        )
    };

    let byte = if byte.as_bytes().len() == 1 {
        byte.as_bytes()[0]
    } else {
        return Err(ParseError::Expected("password policy byte to be exact 1 byte").into());
    };

    let (min, max) = {
        let mut tokens = range.split('-');
        (
            tokens.next().ok_or(ParseError::Expected("policy range (low bound)"))?,
            tokens.next().ok_or(ParseError::Expected("policy range (up bound)"))?,
        )
    };

    let range = (min.parse()?)..=(max.parse()?);
    Ok((PasswordPolicy{byte, range}, password))
}

fn main() -> anyhow::Result<()>{
    let count = include_str!("passwords.txt")
        .lines()
        .map(parse_line)
        .map(Result::unwrap)
        .filter(|(policy, pwd)| policy.is_valid(pwd))
        .count();
    dbg!(count);

    Ok(())
}