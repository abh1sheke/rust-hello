use connection::handle;
use env_local::{EnvErr, VariableType};
use std::net::TcpListener;

pub mod connection;
pub mod env_local;
pub mod thread_pool;

fn main() {
    let vars = env_local::Env::init()
        .unwrap_or_else(|err| match err {
            EnvErr::MissingVar(vars) => {
                panic!("\nMissing the following envars: \n{:?}\n", vars);
            }
        })
        .vars;

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

    let port = {
        if let VariableType::Int(num) = vars.get("POOL_SIZE").unwrap() {
            num
        } else {
            &0
        }
    };
    let listener = TcpListener::bind(format!("127.0.0.1:{}", *port)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle::handle_connection(stream);
        });
    }
    println!("Shutting down.");
}
