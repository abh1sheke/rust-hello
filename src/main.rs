use connection::handle;
use env_local::{EnvErr, VariableType};
use env_logger::TimestampPrecision;
use log::{info, warn, error};
use std::{net::TcpListener, process};
use dotenv::dotenv;

pub mod env_local;
pub mod thread_pool;
pub mod connection;

fn main() {
    dotenv().ok();
    env_logger::builder()
        .format_timestamp(Some(TimestampPrecision::Seconds))
        .init();
    let vars = env_local::Env::init()
        .unwrap_or_else(|err| match err {
            EnvErr::MissingVar(vars) => {
                panic!("\nMissing the following envars: \n{:?}\n", vars);
            }
        })
        .vars;

    let port = {
        if let VariableType::Int(num) = vars.get("PORT").unwrap() {
            num
        } else {
            &0
        }
    };
    let listener = TcpListener::bind(format!("127.0.0.1:{}", *port)).unwrap();
    info!("server listening on port: {}", *port);

    let pool_size = {
        if let VariableType::USize(num) = vars.get("POOL_SIZE").unwrap() {
            num
        } else {
            &0
        }
    };
    let pool = thread_pool::ThreadPool::new(*pool_size).unwrap_or_else(|err| {
        panic!("Error occured while creating thread pool: {}", err);
    });

    for stream in listener.incoming() {
        let stream = stream.unwrap_or_else(|err| {
            error!("error reading incoming stream; {}", err);
            process::exit(1);
        });
        info!("{:?}", stream);
        pool.execute(move || {
            let mut stream = stream;
            handle::handle_connection(&mut stream);
        });
    }
    warn!("shutting down.");
}
