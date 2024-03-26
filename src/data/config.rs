use std::fs;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use scraper::Selector;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::global_const::DATA_DIR;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfigInfo {
    pub game_version: u32
}

impl AppConfigInfo {
    pub fn new() -> Self{
        AppConfigInfo {
            game_version: 526233
        }
    }

    pub fn to_json(&self) -> Result<String>{
        Ok(serde_json::to_string(&self)?)
    }
    pub fn from_json(json: &str) -> Self {
        let result = serde_json::from_str(json);
        return if result.is_ok() {
            result.unwrap()
        } else {
            Self::new()
        }
    }
    pub fn to_local(&self) -> Result<()>{
        let config_path = PathBuf::new().join(DATA_DIR).join("config.json");
        let content = serde_json::to_string(&self)?;
        fs::write(config_path, &content)?;
        Ok(())
    }
    pub fn from_local() -> Result<Self>{
        let config_path = PathBuf::new().join(DATA_DIR).join("config.json");
        let mut config_info = Self::new();
        if config_path.exists() {
            config_info = serde_json::from_str::<AppConfigInfo>(&read_to_string(config_path)?)?;
        } else {
            config_info.to_local()?;
        }
        Ok(config_info)
    }
    pub fn refresh_game_version(&mut self) -> Result<u32>{
        let latest_version = get_latest_version()?;
        if &latest_version > &self.game_version {
            self.game_version = latest_version;
            self.to_local()?;
        }
        Ok(latest_version)
    }
}

fn get_latest_version() -> Result<u32>{
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

pub fn refresh_version() -> Result<u32> {
    let latest_version = get_latest_version()?;
    let mut app_config = AppConfigInfo::new();
    if &latest_version > &app_config.game_version {
        app_config.game_version = latest_version;
        app_config.to_local()?;
    }
    Ok(latest_version)
}

pub fn info_store_current_project(project_info:String) -> Result<()>{
    fs::write(Path::new("./curr_proj.buf"), project_info)?;
    Ok(())
}

pub fn read_current_project_buffer() -> Result<String>{
    Ok(fs::read_to_string(Path::new("./curr_proj.buf"))?)
}