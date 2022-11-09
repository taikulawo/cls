use std::{path::PathBuf, process::ExitStatus};

use lazy_static::lazy_static;
use serde::Deserialize;

use crate::clash_config::ClashAppConfig;

// clash log不存文件，从stdout，stderr直接输出
// 用debug不占用磁盘，方便排除问题
pub const CLASH_LOG_LEVEL: &str = "debug";

pub const DATA_FOLDER: &[u8] = include_bytes!("../data.tar.gz");
const CONFIG_DIR: &str = "cls-config";
pub const CLASH_CONFIG_FILE_NAME: &str = "config.yaml";
pub const CLS_BINARY: &[u8] = include_bytes!("../data/cls");
pub type RunStatus = (ExitStatus, String);

lazy_static! {
    pub static ref OVERRIDE: ClashAppConfig =
        serde_yaml::from_str(include_str!("../data/merge.yaml")).expect("merge conf success");
    pub(crate) static ref CLS_BINARY_LOC: PathBuf =
        dirs::home_dir().expect("no home found").join(".config");
    pub static ref CLASH_CONFIG_FOLDER_PATH: PathBuf = CLS_BINARY_LOC.join(CONFIG_DIR);
    pub(crate) static ref CLASH_SYSTEMD_TEMPLATE: PathBuf =
        CLASH_CONFIG_FOLDER_PATH.join("./cls.service.template");
    pub(crate) static ref CLASH_BINARY_PATH: PathBuf = CLASH_CONFIG_FOLDER_PATH.join("cls");
    pub(crate) static ref PROXYCHAINS_CONF_PATH: PathBuf =
        CLASH_CONFIG_FOLDER_PATH.join("pxychains.conf");
    pub(crate) static ref CLASH_CONFIG_PATH: PathBuf =
        CLASH_CONFIG_FOLDER_PATH.join(CLASH_CONFIG_FILE_NAME);
    pub(crate) static ref CONFIG: Config =
        serde_json::from_str(include_str!("../config.json")).unwrap();
    pub(crate) static ref SYSTEMD_SERVICE_NAME: &'static str = "cls-linux.service";
    pub(crate) static ref SERVICE_DIR: PathBuf = CLS_BINARY_LOC.join("/etc/systemd/system");
    pub(crate) static ref SERVICE_PATH: PathBuf = SERVICE_DIR.join(*SYSTEMD_SERVICE_NAME);
}

#[derive(Deserialize, Clone)]
pub(crate) struct Config {
    pub subscribe_url: String,
}

pub const PROXY_GROUP: &str = r#"
    - { name: reject, type: select, proxies: [REJECT] }
    - { name: private, type: select, proxies: [DIRECT] }
    - { name: icloud, type: select, proxies: [] }
    - { name: apple, type: select, proxies: [] }
    - { name: google, type: select, proxies: [] }
    - { name: gfw, type: select, proxies: [] }
    - { name: tld-not-cn, type: select, proxies: [] }
    - { name: telegramcidr, type: select, proxies: [] }
    - { name: cncidr, type: select, proxies: [] }
    - { name: lancidr, type: select, proxies: [] }
    - { name: applications, type: select, proxies: [] }
    "#;
