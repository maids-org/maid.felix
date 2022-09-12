use crate::types::TimeTable;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Instance {
    /// Blocking client to make requests with.
    pub(crate) client: Client,
    /// Key-value pairs of groups with their ids.
    ///
    /// Each group has its own id on the university website and it is used to
    /// make a get request to the group's timetable.
    pub(crate) codes: HashMap<String, String>,
}

impl Instance {
    /// Construct a new instace of Instance struct with a Client signed in to
    /// the university website using the provided credentials.
    pub fn new(username: &str, password: &str) -> Instance {
        let client = reqwest::blocking::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();

        client
            .post("https://intranet.wiut.uz/Account/Login?ReturnUrl=%2f")
            .form(&[("UserID", username), ("Password", password)])
            .send()
            .unwrap();

        let mut codes: HashMap<_, _> = HashMap::new();

        let table = client
            .get("https://intranet.wiut.uz/TimeTableNew/GetLessons")
            .send()
            .unwrap()
            .text()
            .unwrap();

        let document = Html::parse_document(&table);
        let selector = &Selector::parse("select#ddlclass > option.dropdown-item").unwrap();

        let collection_course: Vec<&str> =
            document.select(selector).flat_map(|el| el.text()).collect();

        let collection_codes: Vec<&str> = document
            .select(selector)
            .flat_map(|el| el.value().attr("value"))
            .collect();

        // stop at undergraduate as theres no code to handle 18:30-21:30 time slot
        // plus the group names for master's don't follow the same pattern
        for index in 0..=179 {
            codes.insert(
                collection_course[index].to_string(),
                collection_codes[index].to_string(),
            );
        }

        Instance { client, codes }
    }

    /// Obtain timetable for one specific group (eg "6BIS6").
    pub fn get_timetable(&self, group: &str) -> TimeTable {
        let response = &self
            .client
            .get(format!(
                "https://intranet.wiut.uz/TimeTableNew/GetLessons?classid={}",
                &self.codes.get(group).unwrap()
            ))
            .send()
            .unwrap()
            .text()
            .unwrap();

        let document = Html::parse_document(&response);

        TimeTable::parse(document)
    }
}
