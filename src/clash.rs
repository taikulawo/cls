use std::{
    fs,
    io::{self, Read},
    os::unix::process::CommandExt,
    path::Path,
    process::{Command as StdCommand, Stdio},
};

use handlebars::Handlebars;
use log::debug;
use serde_json::json;
use tempfile::NamedTempFile;

use crate::{
    clash_config::{ClashAppConfig, ProxyGroup, ProxyGroups},
    commands::{GenerateCommand, ServerCommand},
    config::{
        CLASH_BINARY_PATH, CLASH_CONFIG_FOLDER_PATH, CLASH_CONFIG_PATH, CLASH_SYSTEMD_TEMPLATE,
        CLS_BINARY, DATA_FOLDER, OVERRIDE, PROXYCHAINS_CONF_PATH, SERVICE_DIR, SERVICE_PATH,
        SYSTEMD_SERVICE_NAME,
    },
    updater::update_from_remote,
    utils::run_in_bash_output,
};
const DEFAULT_PROXY_NAME: &str = "Proxy";
const MANUAL_NAME: &str = "Manual";
const AUTO_URLTEST_NAME: &str = "Auto-UrlTest";
pub fn perform_merge(
    mut base: ClashAppConfig,
    remote: String,
    tun: bool,
) -> anyhow::Result<ClashAppConfig> {
    let remote: ClashAppConfig = serde_yaml::from_str(&remote)?;
    if !tun {
        base.tun = None;
    }
    let mut all_target_name = vec![];
    remote.proxies.iter().for_each(|f| {
        if let Some(n) = f.get("name") {
            if let Some(n) = n.as_str() {
                all_target_name.push(n.into());
            }
        }
    });
    let all_proxy_names = all_target_name.clone();
    all_target_name.push("DIRECT".into());
    all_target_name.push("REJECT".into());
    // 生成proxy-groups
    // 在proxies-group开头添加 default Proxy
    let proxy_group: ProxyGroup = ProxyGroup {
        name: DEFAULT_PROXY_NAME.into(),
        proxies: vec![AUTO_URLTEST_NAME.into(), MANUAL_NAME.into()],
        r#type: "select".into(),
        ..Default::default()
    };
    let autotestprofile = ProxyGroup {
        name: AUTO_URLTEST_NAME.into(),
        proxies: all_proxy_names.clone(),
        r#type: "url-test".into(),
        // 以连接youtube的速度为准
        url: Some("https://www.youtube.com/".into()),
        interval: Some("3600".into()),
        ..Default::default()
    };
    let manual_profile = ProxyGroup {
        name: MANUAL_NAME.into(),
        r#type: "select".into(),
        proxies: all_proxy_names.clone(),
        ..Default::default()
    };
    let mut base_proxy_groups = base.proxy_groups.clone();
    for g in &mut base_proxy_groups {
        let mut p = vec![DEFAULT_PROXY_NAME.to_string()];
        p.extend(all_target_name.clone());
        g.proxies = p;
    }
    let mut groups: ProxyGroups = vec![proxy_group, manual_profile, autotestprofile];
    groups.extend(base_proxy_groups);
    base.proxy_groups = groups;
    base.proxies = remote.proxies;

    remove_all_no_resolve_opt(&mut base);
    Ok(base)
}

pub fn normalize_clash_config(config: String, tun: bool) -> anyhow::Result<()> {
    let mut final_config = perform_merge(OVERRIDE.clone(), config, tun)?;

    final_config.external_ui = Some("./ui".into());
    let config = serde_yaml::to_string(&final_config)?;
    fs::write(CLASH_CONFIG_PATH.as_path(), config)?;
    Ok(())
}

// https://github.com/Dreamacro/clash/pull/375#issuecomment-1681954517
pub fn remove_all_no_resolve_opt(config: &mut ClashAppConfig) {
    let rules = &mut config.rules;
    for rule in rules {
        if rule.contains("no-resolve") {
            let a: Vec<&str> = rule
                .split(',')
                .filter(|f| !f.contains("no-resolve"))
                .collect();
            *rule = a.join(",").into();
        }
    }
}

pub fn clash_bootstrap_cli() -> String {
    format!(
        "{}/cls -d {}",
        CLASH_CONFIG_FOLDER_PATH.display(),
        CLASH_CONFIG_FOLDER_PATH.display()
    )
}

pub fn read_from_clash_config_directory<T: AsRef<Path>>(path: T) -> anyhow::Result<String> {
    fs::read_to_string(path).map_err(anyhow::Error::from)
}

pub fn install_all(tun: bool, update: bool) -> anyhow::Result<()> {
    presetup(tun, update)?;

    install_systemd()?;

    Ok(())
}

fn presetup(tun: bool, update: bool) -> anyhow::Result<()> {
    // first, clean all assets
    clean_all()?;
    extract_data()?;
    if update {
        update_from_remote()?;
    }
    install_proxychains()?;

    let config = read_from_clash_config_directory(CLASH_CONFIG_PATH.as_path())?;
    normalize_clash_config(config, tun)?;
    Ok(())
}

pub fn start_proxy_in_foregroud(server: ServerCommand) -> anyhow::Result<()> {
    presetup(server.tun, server.update)?;
    // run process in current context
    let cli = clash_bootstrap_cli();
    StdCommand::new("/bin/bash").args(["-c", &cli]).exec();
    unreachable!("exec unreachable!")
}

pub fn generate(args: GenerateCommand) -> anyhow::Result<()> {
    let mut stdin = io::stdin();
    let mut remote = vec![];
    stdin.read_to_end(&mut remote)?;
    let remote = String::from_utf8(remote)?;
    let result = perform_merge(OVERRIDE.clone(), remote, args.tun)?;
    let result = serde_yaml::to_string(&result)?;
    print!("{}", result);
    Ok(())
}

fn clean_config_directory() -> io::Result<()> {
    //为了安全，只允许移除 cls cwd下的配置目录
    let res = fs::remove_dir_all(CLASH_CONFIG_FOLDER_PATH.as_path());
    if let Err(err) = res {
        if err.kind() != io::ErrorKind::NotFound {
            return Err(err);
        }
    }
    Ok(())
}
pub fn remove_systemd_service() -> anyhow::Result<()> {
    if let Err(err) = fs::remove_file(&*SERVICE_PATH) {
        if err.kind() != io::ErrorKind::NotFound {
            return Err(err.into());
        }
    }
    run_in_bash_output(&format!(
        "systemctl stop {};
        systemctl disable {};
        systemctl daemon-reload;
        systemctl reset-failed;",
        *SYSTEMD_SERVICE_NAME, *SYSTEMD_SERVICE_NAME
    ))?;
    Ok(())
}

pub fn install_proxychains() -> anyhow::Result<()> {
    debug!(
        "pxychains file path {}",
        PROXYCHAINS_CONF_PATH.as_path().display()
    );
    run_in_bash_output(&format!(
        "cat {} > /etc/proxychains.conf",
        fs::canonicalize(PROXYCHAINS_CONF_PATH.as_path())?.to_string_lossy()
    ))?;
    Ok(())
}

pub fn clean_all() -> anyhow::Result<()> {
    clean_config_directory()?;

    remove_systemd_service()?;
    Ok(())
}

pub fn extract_data() -> anyhow::Result<()> {
    run_in_bash_output(&format!("mkdir -p {}", CLASH_CONFIG_FOLDER_PATH.display()))?;
    let tmp_file = NamedTempFile::new()?;
    let tmp_file_path = tmp_file.path();
    fs::write(tmp_file_path, DATA_FOLDER)?;
    let p = StdCommand::new("tar")
        .args([
            "zxvf",
            &tmp_file_path.to_string_lossy(),
            "-C",
            CLASH_CONFIG_FOLDER_PATH.to_str().unwrap(),
            "--strip-components=1",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    p.wait_with_output()?;
    // extract cls binary to config folder
    fs::write(&*CLASH_BINARY_PATH, CLS_BINARY)?;
    run_in_bash_output(&format!(
        "chmod u+x {}",
        CLASH_BINARY_PATH.to_str().unwrap()
    ))?;
    Ok(())
}

pub fn install_systemd() -> anyhow::Result<()> {
    let config = fs::read_to_string(CLASH_SYSTEMD_TEMPLATE.as_path())?;
    let reg = Handlebars::new();
    let cli = clash_bootstrap_cli();

    let result = reg.render_template(
        &config,
        &json!({
            "SCRIPT": cli
        }),
    )?;
    debug!("systemd config {} \n{}\n ", SERVICE_PATH.display(), result);
    run_in_bash_output(&format!("mkdir -p {}", SERVICE_DIR.display()))?;
    fs::write(&*SERVICE_PATH, result)?;
    // start it!
    run_in_bash_output(&format!(
        "
        systemctl enable {};
        systemctl start {};
        ",
        *SYSTEMD_SERVICE_NAME, *SYSTEMD_SERVICE_NAME
    ))
    .unwrap();
    Ok(())
}
