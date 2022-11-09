use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SurgeAppConfig {
    pub general: Option<General>,
    pub script: Option<Script>,
    pub proxy: Option<Proxy>,
    #[serde(rename = "proxy-group")]
    pub proxy_group: Option<ProxyGroup>,
}

#[derive(Debug, Deserialize)]
pub struct General {
    pub loglevel: Option<String>,
    #[serde(rename = "bypass-system")]
    pub bypass_system: Option<bool>,
    #[serde(rename = "skip-proxy")]
    pub skip_proxy: Option<String>,
    #[serde(rename = "bypass-tun")]
    pub bypass_tun: Option<String>,
    #[serde(rename = "dns-server")]
    pub dns_server: Option<String>,
    #[serde(rename = "doh-server")]
    pub doh_server: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Script {
    #[serde(rename = "http-request")]
    pub http_request: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Proxy {
    pub direct: Option<String>,
    #[serde(rename = "remaining-traffic")]
    pub remaining_traffic: Option<String>,
    #[serde(rename = "expiration-time")]
    pub expiration_time: Option<String>,
    pub proxies: Option<Vec<ProxyItem>>,
}

#[derive(Debug, Deserialize)]
pub struct ProxyItem {
    pub name: Option<String>,
    pub address: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub tls: Option<bool>,
    #[serde(rename = "vmess-aead")]
    pub vmess_aead: Option<bool>,
    pub ws: Option<bool>,
    #[serde(rename = "ws-path")]
    pub ws_path: Option<String>,
    pub sni: Option<String>,
    #[serde(rename = "ws-headers")]
    pub ws_headers: Option<String>,
    #[serde(rename = "skip-cert-verify")]
    pub skip_cert_verify: Option<bool>,
    pub tfo: Option<bool>,
    #[serde(rename = "udp-relay")]
    pub udp_relay: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ProxyGroup {
    pub proxies: Option<String>,
    pub apple: Option<String>,
    pub microsoft: Option<String>,
    pub telegram: Option<String>,
    pub streaming: Option<String>,
    pub netease: Option<String>,
    #[serde(rename = "streaming-se")]
    pub streaming_se: Option<String>,
    #[serde(rename = "url-test")]
    pub url_test: Option<String>,
    pub hk: Option<String>,
    pub sg: Option<String>,
    pub tw: Option<String>,
    pub jp: Option<String>,
    pub us: Option<String>,
}
