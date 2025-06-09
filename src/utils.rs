use std::{env, fs};
use std::path::{Path, PathBuf};
use std::process::exit;
use log::{error, info, warn};
use crate::project::git::{add_to_commit, commit_change, create_new_repo};

pub fn get_curr_dir() -> PathBuf{
    let curr_dir = env::current_dir();
    if curr_dir.is_err() {
        error!("无法解析当前目录：{:?}", curr_dir.err());
        exit(2);
    }
    curr_dir.unwrap()
}
pub fn clean_lockfile(){
    let curr_dir = get_curr_dir();
    let lockfile_path = curr_dir.join(".lock");
    if !lockfile_path.exists() {
        exit(0);
    }
    let path_str = fs::read_to_string(&lockfile_path).expect("读取 lockfile 失败");
    let path_str = path_str.trim();
    let path = Path::new(path_str);
    if path.exists(){
        fs::remove_dir_all(path).expect("清理目录失败");
    }
    fs::remove_file(&lockfile_path).expect("移除 lockfile 失败");
    let mut back_sln_path = PathBuf::new();
    for entry in walkdir::WalkDir::new(&curr_dir)
        .max_depth(1) // 只遍历当前目录，不进入子目录
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if let Some(ext) = entry.path().extension() {
            if ext == "bak" {
                back_sln_path = back_sln_path.join(entry.path());
            }
        }
    }
    if back_sln_path.is_file(){
        let back_sln_path = back_sln_path.clone();
        let parent = back_sln_path.parent().expect("目录处理失败");
        let sln_name = back_sln_path.file_stem();
        if sln_name.is_none() {
            return;
        }
        let sln_name = sln_name.unwrap();
        let sln_path = parent.join(sln_name);
        fs::copy(back_sln_path,sln_path).expect("恢复原解决方案失败");
    }
    warn!("程序未按预期运行，现已安全退出")
}

pub fn create_new_repo_util(repo_path: PathBuf){
    let result = create_new_repo(repo_path.clone());
    if result.is_err() {
        error!("创建仓库失败 \n {}", &repo_path.display());
        return;
    }
    let add_result = add_to_commit(repo_path.clone());
    if add_result.is_err() {
        error!("仓库创建成功但未成功暂存文件");
        return;
    }
    let commit_result = commit_change(repo_path.clone(),
                                      "init (created by oni-mod-cli)");
    if commit_result.is_err() {
        error!("未能进行提交");
        return;
    }
    info!("初始化仓库成功")
}