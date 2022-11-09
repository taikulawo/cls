use std::process::{Command as StdCommand, Stdio};

use log::debug;

use crate::config::RunStatus;

pub fn run_cmd(binary: &str, args: &[&str]) -> anyhow::Result<RunStatus> {
    let oneline = args.join(" ");
    let p = StdCommand::new(binary)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()?;
    debug!("{} exit with code {}\n", oneline, p.status);
    let stdout = String::from_utf8(p.stdout)?;
    let stderr = String::from_utf8(p.stderr)?;
    if !stderr.is_empty() && !p.status.success() {
        debug!("run {oneline} error at {}\n", stderr);
        return Ok((p.status, stderr));
    }
    Ok((p.status, stdout))
}

pub fn run_in_bash_output(s: &str) -> anyhow::Result<RunStatus> {
    run_cmd("/bin/bash", &["-c", s])
}
