use core::result::Result;
use std::{io::{ErrorKind, self}, fs::File};

use log::{LevelFilter, Level, Log};
use simplelog::WriteLogger;

pub fn create_log_file(path: &str) -> Result<File, io::Error>{
    let path = std::path::Path::new(path);
    let prefix = path.parent().unwrap();
    if let Err(e) = std::fs::create_dir_all(prefix){
        println!("{:#?} {:#?} {:#?}", prefix, path, e);
    }

    match std::fs::OpenOptions::new()
                .append(true)
                .open(path){
                    Ok(f) => Ok(f),
                    Err(error) => {
                        match error.kind() {
                            ErrorKind::NotFound => File::create(path),
                            _ => Err(error)
                        }
                    }
                }
}

pub fn set_global_logger(path: &str, level: LevelFilter){
    let file = match create_log_file(path){
        Ok(file) => file,
        Err(e) => {
            println!("{:#?}", e);
            panic!()
        }
    };
    let _ = WriteLogger::init(level, simplelog::Config::default(), file);
}

pub fn set_hook_logger(path: &str, level: &LevelFilter) -> Box<WriteLogger<File>>{
    let file = match create_log_file(path){
        Ok(file) => file,
        Err(e) => {
            println!("{:#?}", e);
            panic!()
        }
    };
    WriteLogger::new(*level, simplelog::Config::default(), file)
}
