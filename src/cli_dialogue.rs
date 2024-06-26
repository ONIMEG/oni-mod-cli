use std::path::PathBuf;
use dialoguer::{Confirm, Input};
use log::{error, info, warn};
use regex::Regex;
use spinners::{Spinner, Spinners};
use crate::project::csproj::CSProject;
use crate::project::solution::SolutionInfo;
use crate::utils::{create_new_repo_util, get_curr_dir, get_latest_version};
use anyhow::Result;

pub fn create_sln_with_name(name: String){
    let choose_plib = Confirm::new().with_prompt("是否使用 PLib？")
        .default(true).show_default(true).interact();
    if choose_plib.is_err() {
        error!("解析选项失败：{:?}", choose_plib.as_ref().err())
    }
    let choose_plib = choose_plib.unwrap();
    let curr_dir = get_curr_dir();
    let sln = SolutionInfo::new(name.as_str(), curr_dir);
    let confirm_create = Confirm::new().
        with_prompt(format!("将在 {:?} 目录下创建解决方案，是否确认？", &sln.dir))
        .default(true).show_default(true).interact();
    if confirm_create.is_err() {
        error!("无法解析当前交互值：{:?}",confirm_create.err());
        return;
    }
    let confirm_create = confirm_create.unwrap();
    if !confirm_create {
        info!("取消创建解决方案");
        return;
    }
    let confirm_git = Confirm::new().with_prompt("是否创建 git 仓库？")
        .default(false).show_default(true).interact();
    if confirm_git.is_err() {
        error!("无法解析当前交互值：{:?}",confirm_git.err());
        return;
    }
    let confirm_git = confirm_git.unwrap();
    let create_sln_result = sln.create(choose_plib);
    if create_sln_result.is_err() {
        error!("创建解决方案失败：{:?}", create_sln_result.err());
        return;
    }
    if confirm_git {
        create_new_repo_util(sln.dir.clone());
    }
    info!("{}", create_sln_result.unwrap());
}

pub fn create_sln_without_name(){
    let name_input = Input::new().with_prompt("请输入解决方案名称")
        .default(String::from("ONI-Mods")).show_default(true).interact();
    if name_input.is_err() {
        error!("获取名称失败：{:?}", name_input.err());
        return;
    }
    create_sln_with_name(name_input.unwrap());
}

pub fn create_csproj_with_name(name: String){
    let curr_dir = get_curr_dir();
    let mut target_sln_path = PathBuf::new();
    for entry in walkdir::WalkDir::new(&curr_dir)
        .max_depth(1) // 只遍历当前目录，不进入子目录
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if let Some(ext) = entry.path().extension() {
            if ext == "sln" {
               target_sln_path = target_sln_path.join(entry.path());
            }
        }
    }
    if !target_sln_path.is_file() {
        error!("该目录下未找到解决方案文件");
        return;
    }
    let target_sln_name = target_sln_path.file_stem();
    if target_sln_name.is_none() {
        error!("未能解析到解决方案名称");
        return;
    }
    let target_sln_name = target_sln_name.unwrap();
    let target_sln_name = target_sln_name.to_str();
    if target_sln_name.is_none() {
        error!("未获取到解决方案名称");
        return;
    }
    let choose_plib = Confirm::new().with_prompt("是否使用 PLib？")
        .default(true).show_default(true).interact();
    if choose_plib.is_err() {
        error!("解析选项失败：{:?}", choose_plib.as_ref().err())
    }
    let choose_plib = choose_plib.unwrap();
    let curr_dir = curr_dir.parent();
    if curr_dir.is_none() {
        error!("获取父目录失败");
        return;
    }
    let curr_dir = curr_dir.unwrap().to_path_buf();
    let target_sln_name = target_sln_name.unwrap();
    let root_namespace = get_root_namespace(&curr_dir, target_sln_name);
    if root_namespace.is_err() {
        error!("处理根命名空间失败！")
    }
    let root_namespace = root_namespace.unwrap();

    let desc = Input::new()
        .with_prompt("输入模组介绍")
        .default(String::from("缺氧模组")).show_default(true).interact();
    if desc.is_err() {
        error!("解析模组介绍失败：{:?}", desc.err());
        return;
    }
    let desc = desc.unwrap();
    let mut sp = Spinner::new(Spinners::Earth, String::from("获取当前游戏最新版本号"));
    let latest_version = get_latest_version();
    sp.stop();
    if latest_version.is_err() {
        error!("获取游戏版本号失败：{:?}", latest_version);
        return;
    }
    let latest_version = latest_version.unwrap();
    println!("当前获取到的版本号(可能不是最新版本号)：{}", latest_version);
    let sln = SolutionInfo::new(target_sln_name, curr_dir.clone());
    let mut csproj = CSProject::new(name.as_str(), root_namespace.as_str());
    csproj.property_group.last_working_build = latest_version;
    csproj.property_group.description = desc.to_string();
    let csproj_create = csproj.create(&sln, choose_plib);
    if csproj_create.is_err() {
        error!("创建项目失败：{:?}", csproj_create.err());
        return;
    }
    info!("创建项目成功！")
}

pub fn create_csproj_without_name(){
    let name_input = Input::new().with_prompt("请输入项目名称(决定模组名称)")
        .default(String::from("我的缺氧模组")).show_default(true).interact();
    if name_input.is_err() {
        error!("获取名称失败：{:?}", name_input.err());
        return;
    }
    create_csproj_with_name(name_input.unwrap());
}

fn is_alpha_space(name: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z]+$").unwrap();
    re.is_match(name)
}

fn get_root_namespace(curr_dir: &PathBuf, target_sln_name: &str)-> Result<String>{
    let root_namespace_input = Input::new()
        .with_prompt("输入根命名空间(决定 csproj 项目文件名称)")
        .default(String::from("MyNewMod")).show_default(true);
    loop {
        let root_namespace_b = root_namespace_input.clone().interact()?;

        if curr_dir.join(target_sln_name).join(&root_namespace_b).exists() {
            warn!("存在同名项目！");
            continue;
        }
        if is_alpha_space(root_namespace_b.as_str()) {
            return Ok(root_namespace_b);
        } else {
            warn!("命名空间只能包含字母和数字！");
            continue;
        }
    }
}

