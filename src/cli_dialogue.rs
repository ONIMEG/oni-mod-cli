use std::env;
use std::fmt::format;
use dialoguer::{Confirm, Input};
use log::{error, info};
use crate::project::solution::SolutionInfo;
// TODO 增加 PLib 选择选项
pub fn create_sln_with_name(name: String){
    let curr_dir = env::current_dir();
    if curr_dir.is_err() {
        error!("无法解析当前目录：{:?}", curr_dir.err());
        return;
    }
    let curr_dir = curr_dir.unwrap();
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
    let create_sln_result = sln.create();
    if create_sln_result.is_err() {
        error!("创建解决方案失败：{:?}", create_sln_result.err());
        return;
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
