pub mod instance;
pub mod types;

use crate::instance::Instance;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let credentials = (
        std::env::var("LOGIN").unwrap(),
        std::env::var("PASSWORD").unwrap(),
    );

    let instance = Instance::new(credentials.0.as_str(), credentials.1.as_str());

    dbg!(instance);
}
