use std::env::current_exe;
use std::fs;
use std::fs::{create_dir_all, File, read_to_string};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zip::ZipArchive;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SolutionInfo {
    name: String,
    pub path: PathBuf,
    pub dir: PathBuf,
}
impl SolutionInfo {
    /// 创建一个解决方案的信息对象，同时处理一些简单的错误
    pub fn new(name:&str, dir:PathBuf) -> Self{
        let path = dir.join(name).join(format!("{}.sln",name));
        SolutionInfo {
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
        create_dir_all(&self.dir)?;
        // 将压缩包解压
        let mut resource_path = PathBuf::new();
        if cfg!(debug_assertions) {
            resource_path = resource_path.join("C:\\Users\\26216\\code\\rust\\oni-mod-cli\\resource");
        } else {
            let exe_path = current_exe()?;
            let parent = exe_path.parent();
            if parent.is_none(){
                return Err(anyhow!("解析程序路径失败"));
            }
            resource_path = resource_path.join(parent.unwrap()).join("resource");
        }
        let file = File::open(&resource_path.join("build.zip"))?;
        let mut archive = ZipArchive::new(BufReader::new(file))?;
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let out_path = &self.dir.join(file.mangled_name());
            if (&*file.name()).ends_with('/') {
                create_dir_all(&out_path)?;
            } else {
                if let Some(p) = out_path.parent() {
                    create_dir_all(p)?;
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
        Ok("创建解决方案成功")
    }
}