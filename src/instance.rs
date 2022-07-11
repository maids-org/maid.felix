use std::collections::HashMap;
use reqwest::blocking::Client;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct Instance {
    pub(crate) client: Client,
    pub(crate) codes: HashMap<String, String>,
}

impl Instance {
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

        let collection_course: Vec<&str> = document
          .select(selector)
          .flat_map(|el| el.text())
          .collect();

        let collection_codes: Vec<&str> = document
          .select(selector)
          .flat_map(|el| el.value().attr("value"))
          .collect();

        for index in 0..collection_course.len() {
            codes.insert(collection_course[index].to_string(), collection_codes[index].to_string());
        }

        Instance { client, codes }
    }

    pub fn get_timetable(&self, group: &str) {
        println!("{}", &self.codes.get(group).unwrap());
        let response = &self
            .client
            .get(format!(
                "https://intranet.wiut.uz/TimeTableNew/GetLessons?classid={}",
                group
            ))
            .send()
            .unwrap()
            .text()
            .unwrap();
    }
}
