use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tun {
    pub enable: bool,
    pub stack: String,
    #[serde(rename = "dns-hijack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_hijack: Option<Vec<String>>,
    #[serde(default = "default_bool_true")]
    #[serde(rename = "auto-route")]
    pub auto_route: bool,
    #[serde(default = "default_bool_true")]
    #[serde(rename = "auto-redir")]
    pub auto_redir: bool,
    #[serde(default = "default_bool_true")]
    #[serde(rename = "auto-detect-interface")]
    pub auto_detect_interface: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dns {
    #[serde(default = "default_bool_true")]
    pub enable: bool,
    #[serde(default = "default_bool_true")]
    pub ipv6: bool,
    pub listen: String,
    #[serde(default = "default_bool_true")]
    #[serde(rename = "use-hosts")]
    pub use_hosts: bool,
    #[serde(rename = "default-nameserver")]
    pub default_nameserver: Vec<String>,
    pub nameserver: Vec<String>,
    pub fallback: Vec<String>,
    #[serde(rename = "fallback-filter")]
    pub fallback_filter: FallbackFilter,
    #[serde(rename = "nameserver-policy")]
    pub nameserver_policy: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FallbackFilter {
    pub geoip: bool,
    #[serde(rename = "geoip-code")]
    pub geoip_code: String,
    pub domain: Vec<String>,
}

pub type RuleProviders = HashMap<String, RuleProvider>;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuleProvider {
    pub r#type: String,
    pub behavior: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProxyGroup {
    pub name: String,
    pub r#type: String,
    pub proxies: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

pub type Proxy = HashMap<String, Value>;
pub type ProxyGroups = Vec<ProxyGroup>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClashAppConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "external-controller")]
    pub external_controller: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "external-ui")]
    pub external_ui: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "socks-port")]
    pub socks_port: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mixed-port")]
    pub mixed_port: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allow-lan")]
    pub allow_lan: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default = "default_log_level")]
    #[serde(rename = "log-level")]
    pub log_level: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    pub rules: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tun: Option<Tun>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<Dns>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rule-providers")]
    pub rule_providers: Option<RuleProviders>,
    #[serde(rename = "proxy-groups")]
    pub proxy_groups: ProxyGroups,
    pub proxies: Vec<Proxy>,
}

fn default_log_level() -> String {
    "debug".into()
}
fn default_bool_true() -> bool {
    true
}
