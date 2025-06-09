use crate::resources;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SolutionInfo {
    pub name: String,
    pub path: PathBuf,
    pub dir: PathBuf,
}
impl SolutionInfo {
    /// 创建一个解决方案的信息对象，同时处理一些简单的错误
    pub fn new(name: &str, dir: PathBuf) -> Self {
        let path = dir.join(name).join(format!("{}.sln", name));
        SolutionInfo {
            name: String::from(name),
            path,
            dir: dir.join(name),
        }
    }
    /// 在本地进行解决方案的创建
    pub fn create(&self) -> Result<&str> {
        if self.dir.exists() {
            return Err(anyhow!(
                "检测到解决方案同名文件夹，请删除或者重新选择位置。"
            ));
        }
        // 创建存放项目的文件夹
        fs::create_dir_all(&self.dir)?;
        let lockfile_path = &self.dir.join(".lock");
        fs::write(
            &lockfile_path,
            String::from(self.dir.to_str().unwrap()).as_bytes(),
        )?;
        let sln_name = format!("{}.sln", &self.name);
        let path_sln = &self.dir.join(sln_name);
        let path_gitignore = &self.dir.join(resources::gitignore::FILE_NAME);
        let path_props = &self.dir.join(resources::props::FILE_NAME);
        let path_props_default = &self.dir.join(resources::props_default::FILE_NAME);
        let path_target = &self.dir.join(resources::targets::FILE_NAME);

        let random_guid = Uuid::new_v4().as_hyphenated().to_string().to_uppercase();
        let content_sln = resources::solution::CONTENT.replace("$[guid]", &random_guid);

        fs::write(path_sln, content_sln)?;
        fs::write(path_gitignore, resources::gitignore::CONTENT)?;
        fs::write(path_props, resources::props::CONTENT)?;
        fs::write(path_props_default, resources::props_default::CONTENT)?;
        fs::write(path_target, resources::targets::CONTENT)?;

        fs::remove_file(lockfile_path)?;
        Ok("创建解决方案成功")
    }
}
