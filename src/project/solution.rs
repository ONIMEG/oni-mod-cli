use std::fs;
use std::fs::{create_dir_all, File, read_to_string};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use log::warn;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zip::ZipArchive;

use crate::global_const::DATA_DIR;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SolutionInfo {
    hash: String,
    name: String,
    pub path: PathBuf,
    pub dir: PathBuf,
}
impl SolutionInfo {
    /// 创建一个解决方案的信息对象，同时处理一些简单的错误
    pub fn new(name:&str, dir:PathBuf) -> Self{
        let path = dir.join(name).join(format!("{}.sln",name));
        SolutionInfo {
            hash: String::from(""),
            name: String::from(name),
            path,
            dir: dir.join(name),
        }
    }
    /// 在本地进行解决方案的创建
    pub fn create(&self) -> Result<&str>{
        if self.dir.exists() {
            return Err(anyhow!("检测到解决方案同名文件夹，请删除或者重新选择位置。"));
        }
        // 创建存放项目的文件夹
        fs::create_dir_all(&self.dir)?;
        // 将压缩包解压
        let file = File::open(".\\resource\\build.zip")?;
        let mut archive = ZipArchive::new(BufReader::new(file))?;
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let out_path = &self.dir.join(file.mangled_name());
            if (&*file.name()).ends_with('/') {
                fs::create_dir_all(&out_path)?;
            } else {
                if let Some(p) = out_path.parent() {
                    fs::create_dir_all(p)?;
                }
                let mut out_file = BufWriter::new(File::create(&out_path)?);
                std::io::copy(&mut file, &mut out_file)?;
            }
        }
        // 重命名 solution
        let target_sln = self.dir.join(format!("{}.sln",&self.name));
        fs::rename(&self.dir.join("solution"), &target_sln)?;
        // 修改 solution guid
        let mut content = read_to_string(&target_sln)?;
        let random_guid = Uuid::new_v4().as_hyphenated().to_string().to_uppercase();
        content = content.replace("$[guid]", &random_guid);
        fs::write(&(&target_sln), content)?;
        let save_to_data_result = self.to_local()?;
        return if save_to_data_result {
            Ok("创建项目成功")
        } else {
            fs::remove_dir_all(&self.dir)?;
            Err(anyhow!("创建解决方案失败"))
        }
    }
    pub fn set_hash(&self, hash:String) -> Self {
        return SolutionInfo{
            name: self.name.clone(),
            path: self.path.clone(),
            hash,
            dir: self.dir.clone()
        };
    }
    /// 将对象转换为 `&str` 格式的 json 串
    pub fn to_json(&self) -> Result<String>{
        Ok(serde_json::to_string(&self)?)
    }
    /// 将 json 串转换为解决方案对象
    pub fn from_json(json: &str) -> Self {
        let result = serde_json::from_str(json);
        return if result.is_ok() {
            result.unwrap()
        } else {
            warn!("将 json 串转换为解决方案对象失败！{:?}", result);
            Self::new("None", PathBuf::new())
        }
    }
    /// 在本地存储 solution 条目
    pub fn to_local(&self) -> Result<bool>{
        let digest = md5::compute(&self.to_json()?.as_bytes());
        let hash_str = format!("{:x}", digest);
        let path = PathBuf::new().join("./").join(DATA_DIR).join("sln");
        if !path.exists() {
            create_dir_all(&path)?
        }
        let json = &self.set_hash(hash_str.clone()).to_json()?;
        fs::write(path.join(&hash_str),json)?;
        Ok(true)
    }
}