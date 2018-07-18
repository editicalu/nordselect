extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Deserialize, Debug)]
enum Category {
    Standard,
    P2P,
    Obfuscated,
    Dedicated,
    Tor,
}

#[derive(Deserialize, Debug)]
struct Server {
    flag: String,
    domain: String,
    pub load: u8,
    //categories: Vec<Category>,
}

fn main() {
    let mut data: Vec<Server> = serde_json::from_str(
        &reqwest::get("https://api.nordvpn.com/server")
            .unwrap()
            .text()
            .unwrap(),
    ).unwrap();

    // TODO
    // Filter servers that are not required.

    // Sort the data on load
    data.sort_unstable_by(|x, y| x.load.cmp(&y.load));

    println!("{:?}", data);
}
