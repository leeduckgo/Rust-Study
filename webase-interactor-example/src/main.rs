extern crate pretty_env_logger;

use webase_interactor::Chain;

#[macro_use] extern crate log;

fn main(){
    pretty_env_logger::init(); 
    print_block_number();
    
}

pub fn print_block_number() {
    let ip = "http://127.0.0.1:5002/".to_string();
    let chain = Chain::new(ip);
    let res = chain.get_block_number();
    match res {
        Err(e) => {
            println!("error: {}", e);
        }
        Ok(b_number) => {
            info!("last block height: {}", b_number);
        }
    }
}