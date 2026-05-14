/// Proxy file parsing utilities.
///
/// Format: `host:port:user:pass` (one per line).
/// Lines starting with `#` and blank lines are skipped.
/// Also accepts `host:port` (no auth).
use crate::error::FetchError;

/// Parse a single proxy line into an HTTP proxy URL.
///
/// Accepts two formats:
/// - `host:port:user:pass` -> `http://user:pass@host:port`
/// - `host:port` -> `http://host:port`
pub fn parse_proxy_line(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.trim().splitn(4, ':').collect();
    match parts.len() {
        4 => Some(format!(
            "http://{}:{}@{}:{}",
            parts[2], parts[3], parts[0], parts[1]
        )),
        2 => Some(format!("http://{}:{}", parts[0], parts[1])),
        _ => None,
    }
}

/// Load proxies from a file, returning parsed HTTP proxy URLs.
///
/// Skips blank lines and `#` comments. Returns an error if the file
/// can't be read or contains no valid entries.
pub fn parse_proxy_file(path: &str) -> Result<Vec<String>, FetchError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| FetchError::Build(format!("failed to read proxy file: {e}")))?;

    let proxies: Vec<String> = content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                None
            } else {
                parse_proxy_line(trimmed)
            }
        })
        .collect();

    if proxies.is_empty() {
        return Err(FetchError::Build(
            "proxy file is empty or has no valid entries".into(),
        ));
    }

    Ok(proxies)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn parse_host_port_user_pass() {
        let result = parse_proxy_line("proxy.example.com:8080:alice:s3cret");
        assert_eq!(
            result.as_deref(),
            Some("http://alice:s3cret@proxy.example.com:8080")
        );
    }

    #[test]
    fn parse_host_port_only() {
        let result = parse_proxy_line("10.0.0.1:3128");
        assert_eq!(result.as_deref(), Some("http://10.0.0.1:3128"));
    }

    #[test]
    fn parse_trims_whitespace() {
        let result = parse_proxy_line("  host:9999:user:pass  ");
        assert_eq!(result.as_deref(), Some("http://user:pass@host:9999"));
    }

    #[test]
    fn parse_invalid_returns_none() {
        assert!(parse_proxy_line("just-a-hostname").is_none());
        assert!(parse_proxy_line("a:b:c").is_none()); // 3 parts is invalid
        assert!(parse_proxy_line("").is_none());
    }

    #[test]
    fn parse_file_happy_path() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("proxies.txt");
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "# residential pool").unwrap();
        writeln!(f, "host1:8080:user1:pass1").unwrap();
        writeln!(f).unwrap(); // blank line
        writeln!(f, "host2:3128").unwrap();
        writeln!(f, "# datacenter").unwrap();
        writeln!(f, "host3:9999:u:p").unwrap();
        drop(f);

        let proxies = parse_proxy_file(path.to_str().unwrap()).unwrap();
        assert_eq!(proxies.len(), 3);
        assert_eq!(proxies[0], "http://user1:pass1@host1:8080");
        assert_eq!(proxies[1], "http://host2:3128");
        assert_eq!(proxies[2], "http://u:p@host3:9999");
    }

    #[test]
    fn parse_file_empty_errors() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("empty.txt");
        std::fs::write(&path, "# only comments\n\n").unwrap();

        let err = parse_proxy_file(path.to_str().unwrap());
        assert!(err.is_err());
    }

    #[test]
    fn parse_file_missing_errors() {
        let err = parse_proxy_file("/nonexistent/proxies.txt");
        assert!(err.is_err());
    }
}
