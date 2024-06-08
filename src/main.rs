use std::io;
use reqwest;

pub enum Method {
    GET,
    POST,
}
#[tokio::main]
async fn main() {
    let mut address = String::new();
    let mut method = String::new();
    let mut rest_path = String::new();
    let mut port = String::new();

    // Get the address Components
    println!("Enter address: ");
    io::stdin().read_line(&mut address).expect("Could not read address");

    println!("Enter Port: ");
    io::stdin().read_line(&mut port).expect("Could not read Port");

    println!("Enter path: ");
    io::stdin().read_line(&mut rest_path).expect("Could not read rest");

    println!("Enter method: ");
    io::stdin().read_line(&mut method).expect("Could not read method");




    let full_address = format!("http://{}:{}/{}", address.trim_end(), port.trim_end(), rest_path.trim_end());
    method = method.trim_end().to_lowercase();

    println!("Sending {} request to {}.", method, full_address);

    let client = reqwest::Client::new();

     match method.as_str() {
        "get" => {
            let response = client
                .get(full_address)
                .send()
                .await
                .expect("Could not execute GET request!");
            println!("Status code: {}", response.status());
        }
        "post" => {
            let _response = client
                .post(full_address)
                .send()
                .await
                .expect("Could not execute send request!");
        }
        _ => {
            println!("Invalid method entered!");
        }
    }


}