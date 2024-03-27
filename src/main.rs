use clap::{Parser, Subcommand};
use log::{info, LevelFilter};
use crate::cli_dialogue::{create_sln_with_name, create_sln_without_name};
use std::io::Write;
mod cli_dialogue;
use chrono::Local;
use env_logger::fmt::style;
use env_logger::Env;

mod project;
mod data;
mod global_const;
mod tests;



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

fn handel_create_csproj(name: &Option<String>){

}

fn init_logger(){
    let env = Env::default().filter_or("DEFAULT_FILTER_ENV", "trace");

    // 设置日志打印格式
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}]-{}[{}]{} {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                buf.default_level_style(record.level()),
                record.level(),
                style::AnsiColor::White.on_default(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();
}
fn main() {
    init_logger();
    info!("Logger is up");
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
