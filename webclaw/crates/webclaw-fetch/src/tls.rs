//! Browser TLS + HTTP/2 fingerprint profiles built on wreq (BoringSSL).
//!
//! Replaces the old webclaw-http/webclaw-tls patched rustls stack.
//! Each profile configures TLS options (cipher suites, curves, extensions,
//! PSK, ECH GREASE) and HTTP/2 options (SETTINGS order, pseudo-header order,
//! stream dependency, priorities) to match real browser fingerprints.

use std::time::Duration;

use wreq::http2::{
    Http2Options, PseudoId, PseudoOrder, SettingId, SettingsOrder, StreamDependency, StreamId,
};
use wreq::tls::{AlpsProtocol, CertificateCompressionAlgorithm, TlsOptions, TlsVersion};
use wreq::{Client, Emulation};

use crate::browser::BrowserVariant;
use crate::error::FetchError;

/// Chrome cipher list (TLS 1.3 + TLS 1.2 in Chrome's exact order).
const CHROME_CIPHERS: &str = "TLS_AES_128_GCM_SHA256:TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256:TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256:TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256:TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384:TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384:TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256:TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256:TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA:TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA:TLS_RSA_WITH_AES_128_GCM_SHA256:TLS_RSA_WITH_AES_256_GCM_SHA384:TLS_RSA_WITH_AES_128_CBC_SHA:TLS_RSA_WITH_AES_256_CBC_SHA";

/// Chrome signature algorithms.
const CHROME_SIGALGS: &str = "ecdsa_secp256r1_sha256:rsa_pss_rsae_sha256:rsa_pkcs1_sha256:ecdsa_secp384r1_sha384:rsa_pss_rsae_sha384:rsa_pkcs1_sha384:rsa_pss_rsae_sha512:rsa_pkcs1_sha512";

/// Chrome curves (post-quantum ML-KEM + X25519 + P-256 + P-384).
const CHROME_CURVES: &str = "X25519MLKEM768:X25519:P-256:P-384";

/// Firefox cipher list.
const FIREFOX_CIPHERS: &str = "TLS_AES_128_GCM_SHA256:TLS_CHACHA20_POLY1305_SHA256:TLS_AES_256_GCM_SHA384:TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256:TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256:TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256:TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256:TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384:TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384:TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA:TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA:TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA:TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA:TLS_RSA_WITH_AES_128_GCM_SHA256:TLS_RSA_WITH_AES_256_GCM_SHA384:TLS_RSA_WITH_AES_128_CBC_SHA:TLS_RSA_WITH_AES_256_CBC_SHA";

/// Firefox signature algorithms.
const FIREFOX_SIGALGS: &str = "ecdsa_secp256r1_sha256:ecdsa_secp384r1_sha384:ecdsa_secp521r1_sha512:rsa_pss_rsae_sha256:rsa_pss_rsae_sha384:rsa_pss_rsae_sha512:rsa_pkcs1_sha256:rsa_pkcs1_sha384:rsa_pkcs1_sha512:ecdsa_sha1:rsa_pkcs1_sha1";

/// Firefox curves.
const FIREFOX_CURVES: &str = "X25519MLKEM768:X25519:P-256:P-384:P-521";

/// Safari cipher list.
const SAFARI_CIPHERS: &str = "TLS_AES_128_GCM_SHA256:TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256:TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384:TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256:TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256:TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384:TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256:TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256:TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA:TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA:TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA:TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA:TLS_RSA_WITH_AES_256_GCM_SHA384:TLS_RSA_WITH_AES_128_GCM_SHA256:TLS_RSA_WITH_AES_256_CBC_SHA:TLS_RSA_WITH_AES_128_CBC_SHA";

/// Safari signature algorithms.
const SAFARI_SIGALGS: &str = "ecdsa_secp256r1_sha256:rsa_pss_rsae_sha256:rsa_pkcs1_sha256:ecdsa_secp384r1_sha384:rsa_pss_rsae_sha384:ecdsa_secp521r1_sha512:rsa_pss_rsae_sha512:rsa_pkcs1_sha384:rsa_pkcs1_sha512";

/// Safari curves.
const SAFARI_CURVES: &str = "X25519:P-256:P-384:P-521";

// --- Chrome HTTP headers in correct wire order ---

const CHROME_HEADERS: &[(&str, &str)] = &[
    (
        "sec-ch-ua",
        r#""Google Chrome";v="145", "Chromium";v="145", "Not/A)Brand";v="24""#,
    ),
    ("sec-ch-ua-mobile", "?0"),
    ("sec-ch-ua-platform", "\"Windows\""),
    ("upgrade-insecure-requests", "1"),
    (
        "user-agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36",
    ),
    (
        "accept",
        "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
    ),
    ("sec-fetch-site", "none"),
    ("sec-fetch-mode", "navigate"),
    ("sec-fetch-user", "?1"),
    ("sec-fetch-dest", "document"),
    ("accept-encoding", "gzip, deflate, br, zstd"),
    ("accept-language", "en-US,en;q=0.9"),
    ("priority", "u=0, i"),
];

const CHROME_MACOS_HEADERS: &[(&str, &str)] = &[
    (
        "sec-ch-ua",
        r#""Google Chrome";v="145", "Chromium";v="145", "Not/A)Brand";v="24""#,
    ),
    ("sec-ch-ua-mobile", "?0"),
    ("sec-ch-ua-platform", "\"macOS\""),
    ("upgrade-insecure-requests", "1"),
    (
        "user-agent",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36",
    ),
    (
        "accept",
        "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
    ),
    ("sec-fetch-site", "none"),
    ("sec-fetch-mode", "navigate"),
    ("sec-fetch-user", "?1"),
    ("sec-fetch-dest", "document"),
    ("accept-encoding", "gzip, deflate, br, zstd"),
    ("accept-language", "en-US,en;q=0.9"),
    ("priority", "u=0, i"),
];

const FIREFOX_HEADERS: &[(&str, &str)] = &[
    (
        "user-agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:135.0) Gecko/20100101 Firefox/135.0",
    ),
    (
        "accept",
        "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
    ),
    ("accept-language", "en-US,en;q=0.5"),
    ("accept-encoding", "gzip, deflate, br, zstd"),
    ("upgrade-insecure-requests", "1"),
    ("sec-fetch-dest", "document"),
    ("sec-fetch-mode", "navigate"),
    ("sec-fetch-site", "none"),
    ("sec-fetch-user", "?1"),
    ("priority", "u=0, i"),
];

const SAFARI_HEADERS: &[(&str, &str)] = &[
    (
        "user-agent",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.3.1 Safari/605.1.15",
    ),
    (
        "accept",
        "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
    ),
    ("sec-fetch-site", "none"),
    ("accept-language", "en-US,en;q=0.9"),
    ("sec-fetch-mode", "navigate"),
    ("accept-encoding", "gzip, deflate, br"),
    ("sec-fetch-dest", "document"),
];

const EDGE_HEADERS: &[(&str, &str)] = &[
    (
        "sec-ch-ua",
        r#""Microsoft Edge";v="145", "Chromium";v="145", "Not/A)Brand";v="24""#,
    ),
    ("sec-ch-ua-mobile", "?0"),
    ("sec-ch-ua-platform", "\"Windows\""),
    ("upgrade-insecure-requests", "1"),
    (
        "user-agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36 Edg/145.0.0.0",
    ),
    (
        "accept",
        "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
    ),
    ("sec-fetch-site", "none"),
    ("sec-fetch-mode", "navigate"),
    ("sec-fetch-user", "?1"),
    ("sec-fetch-dest", "document"),
    ("accept-encoding", "gzip, deflate, br, zstd"),
    ("accept-language", "en-US,en;q=0.9"),
    ("priority", "u=0, i"),
];

fn chrome_tls() -> TlsOptions {
    TlsOptions::builder()
        .cipher_list(CHROME_CIPHERS)
        .sigalgs_list(CHROME_SIGALGS)
        .curves_list(CHROME_CURVES)
        .min_tls_version(TlsVersion::TLS_1_2)
        .max_tls_version(TlsVersion::TLS_1_3)
        .grease_enabled(true)
        .permute_extensions(true)
        .enable_ech_grease(true)
        .pre_shared_key(true)
        .enable_ocsp_stapling(true)
        .enable_signed_cert_timestamps(true)
        .alps_protocols([AlpsProtocol::HTTP2])
        .alps_use_new_codepoint(true)
        .aes_hw_override(true)
        .certificate_compression_algorithms(&[CertificateCompressionAlgorithm::BROTLI])
        .build()
}

fn firefox_tls() -> TlsOptions {
    TlsOptions::builder()
        .cipher_list(FIREFOX_CIPHERS)
        .sigalgs_list(FIREFOX_SIGALGS)
        .curves_list(FIREFOX_CURVES)
        .min_tls_version(TlsVersion::TLS_1_2)
        .max_tls_version(TlsVersion::TLS_1_3)
        .grease_enabled(true)
        .permute_extensions(false)
        .enable_ech_grease(true)
        .pre_shared_key(true)
        .enable_ocsp_stapling(true)
        .enable_signed_cert_timestamps(true)
        .certificate_compression_algorithms(&[
            CertificateCompressionAlgorithm::ZLIB,
            CertificateCompressionAlgorithm::BROTLI,
        ])
        .build()
}

fn safari_tls() -> TlsOptions {
    TlsOptions::builder()
        .cipher_list(SAFARI_CIPHERS)
        .sigalgs_list(SAFARI_SIGALGS)
        .curves_list(SAFARI_CURVES)
        .min_tls_version(TlsVersion::TLS_1_2)
        .max_tls_version(TlsVersion::TLS_1_3)
        .grease_enabled(true)
        .permute_extensions(false)
        .enable_ech_grease(false)
        .pre_shared_key(false)
        .enable_ocsp_stapling(true)
        .enable_signed_cert_timestamps(true)
        .certificate_compression_algorithms(&[CertificateCompressionAlgorithm::ZLIB])
        .build()
}

fn chrome_h2() -> Http2Options {
    Http2Options::builder()
        .initial_window_size(6_291_456)
        .initial_connection_window_size(15_728_640)
        .max_header_list_size(262_144)
        .header_table_size(65_536)
        .max_concurrent_streams(1000u32)
        .enable_push(false)
        .settings_order(
            SettingsOrder::builder()
                .extend([
                    SettingId::HeaderTableSize,
                    SettingId::EnablePush,
                    SettingId::MaxConcurrentStreams,
                    SettingId::InitialWindowSize,
                    SettingId::MaxFrameSize,
                    SettingId::MaxHeaderListSize,
                    SettingId::EnableConnectProtocol,
                    SettingId::NoRfc7540Priorities,
                ])
                .build(),
        )
        .headers_pseudo_order(
            PseudoOrder::builder()
                .extend([
                    PseudoId::Method,
                    PseudoId::Authority,
                    PseudoId::Scheme,
                    PseudoId::Path,
                ])
                .build(),
        )
        .headers_stream_dependency(StreamDependency::new(StreamId::zero(), 219, true))
        .build()
}

fn firefox_h2() -> Http2Options {
    Http2Options::builder()
        .initial_window_size(131_072)
        .initial_connection_window_size(12_517_377)
        .max_header_list_size(65_536)
        .header_table_size(65_536)
        .settings_order(
            SettingsOrder::builder()
                .extend([
                    SettingId::HeaderTableSize,
                    SettingId::InitialWindowSize,
                    SettingId::MaxFrameSize,
                ])
                .build(),
        )
        .headers_pseudo_order(
            PseudoOrder::builder()
                .extend([
                    PseudoId::Method,
                    PseudoId::Path,
                    PseudoId::Authority,
                    PseudoId::Scheme,
                ])
                .build(),
        )
        .build()
}

fn safari_h2() -> Http2Options {
    Http2Options::builder()
        .initial_window_size(2_097_152)
        .initial_connection_window_size(10_420_225)
        .max_header_list_size(0)
        .header_table_size(4_096)
        .enable_push(false)
        .max_concurrent_streams(100u32)
        .settings_order(
            SettingsOrder::builder()
                .extend([
                    SettingId::EnablePush,
                    SettingId::MaxConcurrentStreams,
                    SettingId::InitialWindowSize,
                    SettingId::MaxFrameSize,
                ])
                .build(),
        )
        .headers_pseudo_order(
            PseudoOrder::builder()
                .extend([
                    PseudoId::Method,
                    PseudoId::Scheme,
                    PseudoId::Authority,
                    PseudoId::Path,
                ])
                .build(),
        )
        .headers_stream_dependency(StreamDependency::new(StreamId::zero(), 255, false))
        .build()
}

fn build_headers(pairs: &[(&str, &str)]) -> http::HeaderMap {
    let mut map = http::HeaderMap::with_capacity(pairs.len());
    for (name, value) in pairs {
        if let (Ok(n), Ok(v)) = (
            http::header::HeaderName::from_bytes(name.as_bytes()),
            http::header::HeaderValue::from_str(value),
        ) {
            map.insert(n, v);
        }
    }
    map
}

/// Build a wreq Client for a specific browser variant.
pub fn build_client(
    variant: BrowserVariant,
    timeout: Duration,
    extra_headers: &std::collections::HashMap<String, String>,
    proxy: Option<&str>,
) -> Result<Client, FetchError> {
    let (tls, h2, headers) = match variant {
        BrowserVariant::Chrome => (chrome_tls(), chrome_h2(), CHROME_HEADERS),
        BrowserVariant::ChromeMacos => (chrome_tls(), chrome_h2(), CHROME_MACOS_HEADERS),
        BrowserVariant::Firefox => (firefox_tls(), firefox_h2(), FIREFOX_HEADERS),
        BrowserVariant::Safari => (safari_tls(), safari_h2(), SAFARI_HEADERS),
        BrowserVariant::Edge => (chrome_tls(), chrome_h2(), EDGE_HEADERS),
    };

    let mut header_map = build_headers(headers);

    // Append extra headers after profile defaults
    for (k, v) in extra_headers {
        if let (Ok(n), Ok(val)) = (
            http::header::HeaderName::from_bytes(k.as_bytes()),
            http::header::HeaderValue::from_str(v),
        ) {
            header_map.insert(n, val);
        }
    }

    let emulation = Emulation::builder()
        .tls_options(tls)
        .http2_options(h2)
        .headers(header_map)
        .build();

    let mut builder = Client::builder()
        .emulation(emulation)
        .redirect(wreq::redirect::Policy::limited(10))
        .cookie_store(true)
        .timeout(timeout);

    if let Some(proxy_url) = proxy {
        let proxy =
            wreq::Proxy::all(proxy_url).map_err(|e| FetchError::Build(format!("proxy: {e}")))?;
        builder = builder.proxy(proxy);
    }

    builder
        .build()
        .map_err(|e| FetchError::Build(e.to_string()))
}
