pub mod instance;
pub mod types;

use crate::instance::Instance;
use dotenv::dotenv;
use rand::{thread_rng, Rng};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    dotenv().ok();

    let credentials = (
        std::env::var("LOGIN").unwrap(),
        std::env::var("PASSWORD").unwrap(),
    );

    let instance = Instance::new(credentials.0.as_str(), credentials.1.as_str());

    let mut rng = thread_rng();

    for group in instance.codes.keys() {
        println!("Scraping: {}", group);
        let timetable = instance.get_timetable(group);
        println!("{:#?}", timetable);

        // sleep a random number of seconds between each request
        // since we don't want to overwhelm wiut's servers.....right?
        sleep(Duration::from_secs_f64(rng.gen_range(0.5..1.4142)))
    }
}
