use std::fs;

use anyhow::bail;
use reqwest::redirect::Policy;

use crate::config::{CLASH_CONFIG_PATH, CONFIG};

pub(crate) fn update_from_remote() -> anyhow::Result<()> {
    let client = reqwest::blocking::ClientBuilder::new();
    let client = client
        .redirect(Policy::limited(10))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36")
        .build()?;
    let res = client.get(&CONFIG.subscribe_url).send()?;
    let st = res.status();
    let txt: String = res.text()?;
    if !st.is_success() {
        bail!("http error code{}\n {:?}\n", st.as_u16(), txt)
    }
    fs::write(&*CLASH_CONFIG_PATH, txt)?;
    Ok(())
}
