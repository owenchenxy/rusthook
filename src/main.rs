use std::{
    net::{TcpListener},
};

use threadpool;
use rusthook::{config::configs::Configs, mylog::set_global_logger};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// the ip on which the server is listening
   #[arg(short, long, default_value_t = String::from("0.0.0.0"))]
   ip: String,

   /// the port on which the server is listening
   #[arg(short, long, default_value_t = String::from("7878"))]
   port: String,

   /// config file path
   #[arg(short, long, default_value_t = String::from("src/config/hooks.test.yaml"))]
   config: String,

   /// max number of threads
   #[arg(short, long, default_value_t = 4)]
   threads: usize,

   /// stack size for each thread
   #[arg(short, long, default_value_t = 4_000_000)]
   stack_size: usize,
}

fn main() {
    let args = Args::parse();
    let listener = TcpListener::bind(format!("{}:{}", &args.ip, &args.port)).unwrap();

    let pool = threadpool::Builder::new()
    .num_threads(args.threads)
    .thread_name("conn".into())
    .thread_stack_size(args.stack_size).build();
    
    let configs = Configs::new(&args.config);

    //set a global logger
    let global_logger_path = configs.global.get_log_path();
    set_global_logger(&global_logger_path, configs.global.get_log_level().unwrap());

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let cfg = configs.clone();
        pool.execute(||{
            let _ = rusthook::handle_connection(stream, cfg);
        });
    }
    
    pool.join();
}

