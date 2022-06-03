# XTB API

This crate serves as a wrapper for the XTB API http://developers.xstore.pro/documentation/

# Usage

```rs
pub fn main() {
    // New instance of Client with specified ConnectionType
    let mut client = Client::new(xtb_api::models::ConnectionType::Demo);

    // Login in with userId and password
    client.login(12345, String::from("password")).unwrap();
    
    // Get Balance
    let balance = client.balance().unwrap();
}
```