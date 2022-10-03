use lazy_static::lazy_static;
use regex::Regex;
use scraper::{Html, Selector};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Lesson {
    /// Name of the module (eg "Introduction to Statistics and Data Science").
    pub name: String,
    /// Name of the tutor giving the lesson (eg "THE MYTH & THE LEGEND, OLMOS ISAKOV").
    pub tutor: String,
    /// Lesson format. Can be either "lecture", "online lecture", "seminar" or "workshop".
    #[serde(rename = "type")]
    pub format: String,
    /// The time at which the lesson starts.
    pub start: f32,
    /// Length of the lesson.
    pub length: f32,
    /// The room in which the lesson is going to be held.
    pub location: String,
}

impl Lesson {
    /// Construct a new instance of a Lesson given the start time and data, which
    /// at this point should look like:
    /// ["location", "module name_format_blah", "teacher"]
    fn new(start: f32, data: &[&str]) -> Lesson {
        lazy_static! {
            static ref KILL_BRACKETS_RE: Regex = Regex::new(r"\s?\(\s?\d+\s?\)").unwrap();
        }

        let (name, format) = Self::process_class(data[1].trim());
        let tutor = data[2].trim().to_string();
        let location = KILL_BRACKETS_RE.replace(data[0], "").trim().to_string();

        Lesson {
            name,
            tutor,
            format,
            start,
            length: 1.0,
            location,
        }
    }

    /// Return a tuple with class name as the first & class format as the second item.
    /// The input string can be written as:
    /// 1) "Online_module name_format_extra_info".
    /// 2) "online / module name_format_extra_info".
    /// 3) "module name_format_extra_info".
    fn process_class(class: &str) -> (String, String) {
        let mut format = String::new();

        // removing the "online" prefix if it exists.
        // i really dont want to write a separate regex for this.
        // pls dont yell at me.
        let class: &str = if class.to_lowercase().starts_with("online_") {
            format += "online ";
            class.split_once("_").unwrap().1
        } else if class.to_lowercase().starts_with("online /") {
            format += "online ";
            class.split_once("/").unwrap().1
        } else {
            class
        };

        let class: Vec<&str> = class.splitn(2, "_").collect();

        let mut name = class[0].trim().to_string();

        // module name is incomplete on intranet (4BABM module)
        if name.ends_with("Beha") {
            name += "viour"
        }

        if class[1].contains("lec_") {
            format += "lecture"
        } else if class[1].contains("w_") {
            format += "workshop"
        } else {
            format += "seminar"
        }

        (name, format)
    }

    /// Check if the current lesson is a continuation of the given lesson.
    /// This method is used to determine if the lesson needs to be prolonged.
    fn is_continuation(&self, lesson: &Lesson) -> bool {
        (&self.name, &self.format, self.start) == (&lesson.name, &lesson.format, lesson.start + 1.0)
    }

    /// Add one hour to the length of the current lesson.
    fn prolong(&mut self) {
        self.length += 1.0;
    }
}

pub type Day = Vec<Lesson>;

/// Timetable for one group at the universiy.
/// There's one field for every weekday which contains a vector of Lessons for
/// that given day. The vector is empty if there are no Lessons on that day.
#[derive(Serialize, Debug)]
pub struct TimeTable {
    #[serde(rename = "0")]
    pub sunday: Day,
    #[serde(rename = "1")]
    pub monday: Day,
    #[serde(rename = "2")]
    pub tuesday: Day,
    #[serde(rename = "3")]
    pub wednesday: Day,
    #[serde(rename = "4")]
    pub thursday: Day,
    #[serde(rename = "5")]
    pub friday: Day,
    #[serde(rename = "6")]
    pub saturday: Day,
    #[serde(rename = "7")]
    pub another_sunday: Day,
}

impl TimeTable {
    /// Construct a new instance of an empty TimeTable.
    fn new() -> Self {
        TimeTable {
            sunday: Vec::new(),
            monday: Vec::new(),
            tuesday: Vec::new(),
            wednesday: Vec::new(),
            thursday: Vec::new(),
            friday: Vec::new(),
            saturday: Vec::new(),
            another_sunday: Vec::new(),
        }
    }

    /// Construct a new instance of the TimeTable from parsing the html.
    pub fn from_html(document: Html) -> Self {
        let mut timetable = TimeTable::new();

        // first row is excluded as it only has information about the timeslots
        let selector = Selector::parse("div.row.cf:not(:first-of-type)").unwrap();
        let rows = document.select(&selector);

        for (index, row) in rows.enumerate() {
            // first slot is excluded as it only has info on weekdays
            let selector = Selector::parse("div.col:not(:first-of-type) .innerbox").unwrap();
            let slots = row
                .select(&selector)
                .map(|el| el.text().collect::<Vec<&str>>())
                .collect::<Vec<_>>();
            let day = Self::get_day_lessons(slots);

            match index {
                0 => timetable.monday = day,
                1 => timetable.tuesday = day,
                2 => timetable.wednesday = day,
                3 => timetable.thursday = day,
                4 => timetable.friday = day,
                5 => timetable.saturday = day,
                _ => panic!("more than 6 rows found, is wiut having lessons on sunday now?"),
            }
        }

        timetable
    }

    ///  Return a list of lessons for a given day.
    fn get_day_lessons(slots: Vec<Vec<&str>>) -> Day {
        let mut day: Vec<Lesson> = Vec::new();

        // to handle collisions correctly, we need to keep track of the number
        // of lessons in the last time slot
        let mut last_slot_lessons = 0;
        for (offset, slot) in slots.into_iter().enumerate() {
            let lessons = Self::process_slot(slot, offset);
            let number_of_lessons = lessons.len();

            for lesson in lessons {
                let mut lesson_prolonged = false;
                // to check if one of the previous lessons should be prolonged
                let start = match day
                    .len()
                    .checked_sub(number_of_lessons + last_slot_lessons - 1)
                {
                    Some(num) => num,
                    None => 0,
                };

                for index in start..day.len() {
                    if let Some(previous_lesson) = day.get_mut(index) {
                        if lesson.is_continuation(previous_lesson) {
                            previous_lesson.prolong();
                            lesson_prolonged = true;
                            break;
                        }
                    }
                }

                if !lesson_prolonged {
                    day.push(lesson)
                }
            }

            last_slot_lessons = number_of_lessons;
        }

        day
    }

    /// Get a list of Lessons in the slot.
    /// An empty vector will be returned if there are no Lessons.
    fn process_slot(slot: Vec<&str>, offset: usize) -> Vec<Lesson> {
        lazy_static! {
            static ref GROUP_RE: Regex =
                Regex::new(r"\d(CIFS|BABM|BIS|CL|ECwF|Fin|BM(Fin|Mar))\d+").unwrap();
        }
        // get rid of strings with just whitespace and strings that have group names
        let mut data: Vec<&str> = slot
            .into_iter()
            .filter(|text| !(text.trim().is_empty() || GROUP_RE.is_match(text)))
            .collect();

        // for now one class does not have a location (5Fin6 Intro to Crypto)
        if data.len() == 2 {
            data.insert(0, "blockchain");
        }

        let mut lessons: Vec<Lesson> = Vec::new();
        for (index, _) in data.iter().enumerate().step_by(3) {
            lessons.push(Lesson::new(9.0 + offset as f32, &data[index..index + 3]));
        }
        lessons
    }
}
