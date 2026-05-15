#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailedLogin {
    pub user: String,
    pub ip: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseOutcome {
    Failed(FailedLogin),
    Ignored,
    Malformed,
}

#[allow(clippy::collapsible_if)]
pub fn parse_line(line: &str) -> ParseOutcome {
    let line = line.trim();
    if line.is_empty() {
        return ParseOutcome::Ignored;
    }

    if line.contains("Failed password for") || line.contains("Invalid user") {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut user = String::new();
        let mut ip = String::new();

        if let Some(from_pos) = parts.iter().position(|&p| p == "from") {
            if from_pos > 0 {
                user = parts[from_pos - 1].to_string();
            }
            if from_pos + 1 < parts.len() {
                ip = parts[from_pos + 1].to_string();
            }
        } else if line.contains("Invalid user") {
            if let Some(user_pos) = parts.iter().position(|&p| p == "user") {
                if user_pos + 1 < parts.len() {
                    user = parts[user_pos + 1].to_string();
                }
            }
        }

        if user.is_empty() || ip.is_empty() {
            ParseOutcome::Malformed
        } else {
            ParseOutcome::Failed(FailedLogin { user, ip })
        }
    } else if line.contains("Accepted password") {
        ParseOutcome::Ignored
    } else {
        ParseOutcome::Malformed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failed_login() {
        let line =
            "Jan 10 08:15:21 srv01 sshd[1001]: Failed password for root from 1.2.3.4 port 123 ssh2";
        let outcome = parse_line(line);
        if let ParseOutcome::Failed(login) = outcome {
            assert_eq!(login.user, "root");
            assert_eq!(login.ip, "1.2.3.4");
        } else {
            panic!("Devrait être un FailedLogin");
        }
    }

    #[test]
    fn test_accepted_ignored() {
        let line = "Jan 10 08:16:44 srv01 sshd[1003]: Accepted password for student from 192.0.2.15 port 44822 ssh2";
        assert_eq!(parse_line(line), ParseOutcome::Ignored);
    }

    #[test]
    fn test_malformed_line() {
        let line = "MALFORMED LINE WITHOUT EXPECTED SSH FIELDS";
        assert_eq!(parse_line(line), ParseOutcome::Malformed);
    }
}
