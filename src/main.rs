use clap::{Parser, Subcommand};
use log::{LevelFilter};
use crate::cli_dialogue::{
    create_csproj_with_name,
    create_csproj_without_name,
    create_sln_with_name,
    create_sln_without_name
};
use std::io::Write;
use env_logger::fmt::style;

mod project;
mod tests;
mod cli_dialogue;
mod utils;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
     command: Commands,
}

#[derive(Subcommand)]
enum Commands {
 /// 创建解决方案或者项目
 Create {
     #[command(subcommand)]
     create_commands: CreateCommands
 },
}

#[derive(Subcommand)]
enum CreateCommands{
    /// 创建解决方案选项
    Sln {
        /// 解决方案名称，可选
        name: Option<String>
    },
    /// 创建项目选项
    Csproj {
        /// 项目名称，可选
        name: Option<String>
    }
}

fn handel_create_sln(_name: &Option<String>){
    if let Some(value) = _name {
        create_sln_with_name(value.clone())
    } else {
        create_sln_without_name()
    }
}

fn handel_create_csproj(_name: &Option<String>){
    if let Some(value) = _name {
        create_csproj_with_name(value.clone())
    } else {
        create_csproj_without_name()
    }
}

fn init_logger(){
    // 设置日志打印格式
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}{}{}",
                buf.default_level_style(record.level()),
                record.args(),
                style::AnsiColor::White.on_default(),
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}
fn main() {
    init_logger();
     let cli = Cli::parse();
     match &cli.command {
         Commands::Create {create_commands}=> {
            match create_commands {
                CreateCommands::Sln {name} => {
                    handel_create_sln(name)
                },
                CreateCommands::Csproj {name} => {
                    handel_create_csproj(name)
                }
            }
         }
     }
}
