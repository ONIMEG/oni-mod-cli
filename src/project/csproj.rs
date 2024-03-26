use std::{
    fs::{self, File, OpenOptions},
    io::BufReader,
    iter,
    path::PathBuf
};
use std::io::{Read, Write};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use xml::reader::{EventReader, XmlEvent};
use crate::project::solution::SolutionInfo;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CSProject {
    #[serde(rename = "PropertyGroup")]
    pub property_group: PropertyGroup
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PropertyGroup {
    #[serde(rename = "AssemblyTitle")]
    pub assembly_title: String,
    #[serde(rename = "FileVersion")]
    pub file_version: String,
    #[serde(rename = "RootNamespace")]
    pub root_namespace: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "AssemblyVersion")]
    pub assembly_version: String,
    #[serde(rename = "LastWorkingBuild")]
    pub last_working_build: u32,
    #[serde(rename = "Platforms")]
    pub platforms: String,
}

const CS_GUID: &str = "9A19103F-16F7-4668-BE54-9A1E7A4F7556";
const PROJECT_ITEM: &str = r#"Project("{$[a]}") = "$[b]", "$[b]\$[b].csproj", "{$[c]}"
EndProject
"#;

impl CSProject{
    pub fn new(csproj_name: &str, root_name: &str) -> Self {
        CSProject{
            property_group: PropertyGroup{
                assembly_title: String::from(csproj_name),
                file_version: String::from("1.0.0"),
                root_namespace: String::from(root_name),
                description: String::from("缺氧模组"),
                assembly_version: String::from("1.0.0"),
                last_working_build: 526233,
                platforms: String::from("Vanilla;Mergedown"),
            }
        }
    }
    /// 创建本地 csproj
    pub fn create(&self, sln: &SolutionInfo) -> Result<()>{
        let proj_name = &self.property_group.root_namespace;
        let target_dir = &sln.dir.join(proj_name);
        fs::create_dir_all(target_dir)?;
        let target_path = &target_dir.join(format!("{}.csproj",proj_name));
        create_file(&self, target_path)?;
        add_mod_cs(target_dir.join("Mod.cs"), &self)?;
        add_csproj_to_sln(&sln.path, &self.property_group.root_namespace)?;
        Ok(())
    }
}
/// 创建 csproj 文件
fn create_file(csproj: &CSProject, target_path: &PathBuf) -> Result<()>{
    let mut new_csproj_xml = serde_xml_rs::to_string(&csproj)?;
    new_csproj_xml = format_xml(new_csproj_xml)?;
    new_csproj_xml = new_csproj_xml.replace(
        "<Project>",
        "<Project Sdk=\"Microsoft.NET.Sdk\">");
    fs::write(target_path, new_csproj_xml)?;
    Ok(())
}
/// 格式化 xml
fn format_xml(xml_string: String) -> Result<String> {
    let input = BufReader::new(xml_string.as_bytes());
    let mut formatted_xml = String::new();
    let parser = EventReader::new(input);
    let mut depth = 0;
    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, .. }) => {
                formatted_xml += &iter::repeat("  ")
                    .take(depth)
                    .collect::<String>();
                if depth == 2 {
                    formatted_xml += &format!("<{}>", name);
                } else {
                    formatted_xml += &format!("<{}>\n", name);
                }
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                if depth < 2 {
                    formatted_xml += &iter::repeat("  ")
                        .take(depth)
                        .collect::<String>();
                }
                formatted_xml += &format!("</{}>\n", name);
            }
            Ok(XmlEvent::Characters(text)) => {
                formatted_xml += &format!("{}", text.trim());
            }
            Ok(_) => {}
            Err(e) => return Err(anyhow!("格式化 xml 失败：{}",e.to_string())),
        }
    }
    Ok(formatted_xml)
}
/// 创建 Mod.cs
fn add_mod_cs(target_path: PathBuf, new_info: &CSProject) -> Result<()>{
    let mut file_obj = File::open(".\\resource\\Mod.cs").expect("找不到 Mod.cs");
    let mut code = String::new();
    file_obj.read_to_string(&mut code).expect("读取 Mod.cs 失败");
    code = code.replace("{assembly_title}", &new_info.property_group.root_namespace);
    fs::write(target_path,code)?;
    Ok(())
}
// 在 solution 添加 project 字段
fn add_csproj_to_sln(target_sln: &PathBuf, csproj_name: &String) -> Result<()> {
    print!("{:?}", target_sln);
    let project_item = PROJECT_ITEM
        .replace("$[a]", CS_GUID)
        .replace("$[b]", csproj_name)
        .replace("$[c]", &*Uuid::new_v4().as_hyphenated().to_string().to_uppercase());
    let mut sln_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(false)
        .open(&target_sln)?;
    sln_file.write_all(project_item.as_bytes())?;
    Ok(())
}
