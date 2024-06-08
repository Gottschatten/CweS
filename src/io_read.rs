use std::io;

pub struct CRequest {
    ip: String,
    port: String,
    method: String,
    path: String,
    full: String,
}

impl CRequest {
    fn new(ip: String, port: String, method: String, path: String, full: String) -> CRequest {
        CRequest { ip: String::from(ip.trim_end()),
             port: String::from(port.trim_end()), 
             method: String::from(method.trim_end().to_lowercase()), 
             path: String::from(path.trim_end()), 
             full
            }
    }
    pub fn get_method(&self) -> &str {
        &self.method
    }
    pub fn get_full(&self) -> &str {
        &self.full
    }
    pub fn get_port(&self) -> &str {
        &self.port
    }
    pub fn get_ip(&self) -> &str {
        &self.ip
    }
    pub fn get_path(&self) -> &str {
        &self.path
    }
}


pub fn address_input() -> CRequest {
    let mut ip = String::new();
    let mut method = String::new();
    let mut rpath = String::new();
    let mut port = String::new();

    // Get the address Components
    println!("Enter address: ");
    io::stdin().read_line(&mut ip).expect("Could not read address");

    println!("Enter Port: ");
    io::stdin().read_line(&mut port).expect("Could not read Port");

    println!("Enter path: ");
    io::stdin().read_line(&mut rpath).expect("Could not read path");

    println!("Enter method: ");
    io::stdin().read_line(&mut method).expect("Could not read method");

    let full = format!("http://{}:{}/{}", ip.trim_end(), port.trim_end(), rpath.trim_end());

    CRequest::new(ip, port, method, rpath, full)
}


