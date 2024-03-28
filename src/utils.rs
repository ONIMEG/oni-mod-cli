use std::env::current_exe;
use std::path::PathBuf;
use scraper::Selector;
use anyhow::{anyhow, Result};

pub fn get_latest_version() -> Result<u32> {
    let url = "https://forums.kleientertainment.com/game-updates/oni-alpha/";
    let response = reqwest::blocking::get(url);
    if response.is_err() {
        return Ok(526233)
    }
    let body = response?.text().unwrap();
    let document = scraper::Html::parse_document(&body);
    let selector = Selector::parse("h3").expect("选择器无法解析");
    let version_txt = document.select(&selector).next().map(|element| element.text().collect::<String>());
    return if let Some(text) = version_txt {
        let str = text.trim().replace("Release", "");
        Ok(str.trim().parse::<u32>().unwrap())
    } else {
        Ok(526233)
    }
}

pub fn get_resource_path() -> Result<PathBuf>{
    let mut resource_path = PathBuf::new();
    if cfg!(debug_assertions) {
        resource_path = resource_path
            .join("C:\\Users\\26216\\code\\rust\\oni-mod-cli\\resource");
    } else {
        let exe_path = current_exe()?;
        let parent = exe_path.parent();
        if parent.is_none(){
            return Err(anyhow!("解析程序路径失败"));
        }
        resource_path = resource_path.join(parent.unwrap()).join("resource");
    }
    Ok(resource_path)
}