# XTB API

This crate serves as a wrapper for the XTB API http://developers.xstore.pro/documentation/

# Usage

```rs
pub fn main() {
    let mut client = Client::new();

    client.login(12345, String::from("password"), true).unwrap();

    let balance = client.balance().unwrap();
}
```