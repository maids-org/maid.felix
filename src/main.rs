pub mod instance;
pub mod types;

use crate::instance::Instance;
use dotenv::dotenv;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use regex::Regex;
use std::thread::sleep;
use std::time::Duration;
use std::{fs, path::PathBuf};

fn main() {
    dotenv().ok();

    let credentials = (
        std::env::var("LOGIN").unwrap(),
        std::env::var("PASSWORD").unwrap(),
    );

    let instance = Instance::new(credentials.0.as_str(), credentials.1.as_str());

    // delete old timetable data
    let data_path = PathBuf::from("./data/");
    if data_path.exists() {
        fs::remove_dir_all("./data").expect("failed to delete old timetable");
    }

    lazy_static! {
        static ref COURSE_RE: Regex = Regex::new(r"[3-6]\D+").unwrap();
    }
    let mut rng = thread_rng();

    for group in instance.codes.keys() {
        println!("Scraping: {}", group);
        let timetable = instance.get_timetable(group);
        let course = COURSE_RE.find(group).unwrap().as_str();

        let path = data_path.join(course);
        if !path.exists() {
            fs::create_dir_all(&path).expect("failed to create a directory and all of its parents");
        }

        let json = serde_json::to_string_pretty(&timetable).unwrap();
        let path = path.join(format!("{}.json", group));
        fs::write(&path, json).unwrap();

        // sleep a random number of seconds between each request
        // since we don't want to overwhelm wiut's servers.....right?
        sleep(Duration::from_secs_f64(rng.gen_range(0.5..1.4142)))
    }
}
