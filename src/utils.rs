use std::collections::hash_map::DefaultHasher;
use std::env::current_exe;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::process::exit;
use scraper::Selector;
use anyhow::{anyhow, Result};
use log::error;

const BUILD_HASH_PLIB:u64 = 4748454135723726910;
const MOD_HASH_PLIB:u64 = 2910857530418022517;
const MOD_HASH:u64 = 10249465668325030135;

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

pub fn get_resource_path(is_plib:bool) -> Result<PathBuf>{
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
    if is_plib {
        resource_path = resource_path.join("plib");
    }
    Ok(resource_path)
}

pub fn compare_hash()->Result<()>{
    let hash_list:[u64; 3] = [BUILD_HASH_PLIB, MOD_HASH_PLIB, MOD_HASH];
    let file_list:[&str; 3] = ["plib/build.zip","plib/Mod.cs","Mod.cs"];
    let root_path = get_resource_path(false)?;
    for i in 0..3 {
        println!("[{}/{}]检查文件完整性", i+1, hash_list.len());
        let file_path = root_path.join(file_list[i]);
        let data = fs::read(&file_path)?;
        let curr_hash = calculate_hash(&data);
        if curr_hash != hash_list[i] {
            error!("程序文件可能被破坏，请重新安装本程序");
            exit(1);
        }
    }
    Ok(())
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}