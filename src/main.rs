use cwes::{io_read::address_input, request::cwes_request};
#[tokio::main]
async fn main() {
    let address = address_input();

    println!("Sending {} request to {}.", address.get_method(), address.get_full());

    let status = cwes_request(address).await;

    match status {
        Ok(s) => println!("{}",s),
        Err(e) => eprintln!("{}",e)
    }

}