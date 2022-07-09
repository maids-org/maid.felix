use crate::types::Code;
use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

#[derive(Debug)]
pub struct Instance {
    pub(crate) client: Client,
    pub(crate) codes: Vec<Code>,
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

        let mut codes: Vec<(_, _)> = vec![];

        let table = client
            .get("https://intranet.wiut.uz/TimeTableNew/GetLessons")
            .send()
            .unwrap()
            .text()
            .unwrap();

        let document = Html::parse_document(&table);
        let selector = &Selector::parse(".dropdown1").unwrap();

        let childs = document.select(selector).map(|el| println!("{:?}",el));

        // println!("{:?}", childs);

        Instance { client, codes }
    }

    pub fn get_timetable(&self, id: &str) {
        let response = &self
            .client
            .get(format!(
                "https://intranet.wiut.uz/TimeTableNew/GetLessons?classid={}",
                id
            ))
            .send()
            .unwrap()
            .text()
            .unwrap();

        let parsed = Html::parse_document(response);

        println!("{:?}", parsed)
    }
}
