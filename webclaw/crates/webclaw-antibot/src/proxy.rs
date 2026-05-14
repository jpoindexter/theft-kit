use std::sync::Mutex;

struct Inner {
    proxies: Vec<String>,
    index: usize,
}

/// Round-robin proxy pool with thread-safe rotation.
pub struct ProxyPool {
    inner: Mutex<Inner>,
}

impl ProxyPool {
    /// Create from a single proxy URL and/or a list. Returns None if empty.
    pub fn new(single: Option<&str>, list: &[String]) -> Option<Self> {
        let mut proxies: Vec<String> = Vec::new();
        if !list.is_empty() {
            proxies = list.to_vec();
        } else if let Some(s) = single {
            if !s.is_empty() {
                proxies.push(s.to_string());
            }
        }
        if proxies.is_empty() {
            return None;
        }
        log::info!("Proxy pool: {} proxies configured", proxies.len());
        Some(Self {
            inner: Mutex::new(Inner { proxies, index: 0 }),
        })
    }

    /// Current proxy without rotating.
    pub fn current(&self) -> String {
        let s = self.inner.lock().unwrap();
        s.proxies[s.index % s.proxies.len()].clone()
    }

    /// Advance to next proxy and return it.
    pub fn rotate(&self) -> String {
        let mut s = self.inner.lock().unwrap();
        s.index += 1;
        let proxy = s.proxies[s.index % s.proxies.len()].clone();
        log::info!("Connection refreshed");
        proxy
    }

    pub fn len(&self) -> usize {
        self.inner.lock().unwrap().proxies.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Redact proxy URL for safe logging.
pub fn mask_proxy(url: &str) -> String {
    if url.len() > 15 {
        if let Some(i) = url.find("://") {
            return format!("{}***{}", &url[..i + 3], &url[url.len() - 4..]);
        }
        return format!("***{}", &url[url.len() - 4..]);
    }
    "***".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_rotates() {
        let pool = ProxyPool::new(
            None,
            &["http://a:1".into(), "http://b:2".into(), "http://c:3".into()],
        )
        .unwrap();
        assert_eq!(pool.current(), "http://a:1");
        assert_eq!(pool.rotate(), "http://b:2");
        assert_eq!(pool.rotate(), "http://c:3");
        assert_eq!(pool.rotate(), "http://a:1"); // wraps
    }

    #[test]
    fn mask_hides_middle() {
        assert_eq!(
            mask_proxy("http://user:pass@proxy.example.com:8080"),
            "http://***8080"
        );
    }

    #[test]
    fn empty_returns_none() {
        assert!(ProxyPool::new(None, &[]).is_none());
    }
}
