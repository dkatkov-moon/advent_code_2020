
#[cfg(test)]
mod tests {
    use super::PasswordPolicy;

    #[test]
    fn test_is_valid() {
        let pp = PasswordPolicy {
            byte: b'a',
            positions: [0,2],
        };

        assert_eq!(pp.is_valid("zeus"), false);
        assert_eq!(pp.is_valid("abcd"), true);
        assert_eq!(pp.is_valid("qbad"), true);
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
                positions: [0,2]
            },
             "banana")
        );
    }
}

#[derive(PartialEq, Debug)]
struct PasswordPolicy {
    byte: u8,
    positions: [usize; 2]
}

impl PasswordPolicy {
    fn is_valid(&self, password: &str) -> bool {
        self.positions
        .iter()
        .copied()
        .filter(|&index| password.as_bytes()[index] == self.byte)
        .count() == 1
    }
}

fn parse_line(s: &str) -> anyhow::Result<(PasswordPolicy, &str)> {
    peg::parser!{
        grammar parser() for str {
          rule number() -> usize
            = n:$(['0'..='9']+) { n.parse().unwrap() }
      
          rule position() -> usize
            = n:number() {n - 1}

          rule positions() -> [usize; 2]
            = first:position() "-" second:position() { [first, second] }

          rule byte() -> u8
            = letter:$(['a'..='z']) { letter.as_bytes()[0] }
            
          rule password() -> &'input str
            = letters:$([_]*) { letters }

          pub(crate) rule line() -> (PasswordPolicy, &'input str)
            = positions:positions() " " byte:byte() ": " password:password() {
              (PasswordPolicy { byte, positions }, password)
          }
        }
      }
    Ok(parser::line(s)?)
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