use lazy_static::lazy_static;
use regex::Regex;
use scraper::{Html, Selector};
use serde::Serialize;

#[derive(Serialize)]
pub struct Lesson {
    /// Name of the module (eg "Introduction to Statistics and Data Science").
    pub name: String,
    /// Name of the tutor giving the lesson (eg "THE MYTH & THE LEGEND, OLMOS ISAKOV").
    pub tutor: String,
    /// Lesson format. Can be either "lecture", "online lecture", "seminar" or "workshop".
    pub format: String,
    /// The time at which the lesson starts.
    pub start: f32,
    /// Length of the lesson.
    pub length: f32,
    /// The room in which the lesson is going to be held.
    pub location: String,
}

impl Lesson {
    /// Construct a new instance of a Lesson given the timetable slot and the index
    /// position of the slot (which is used for indicating start time).
    fn new(slot: Vec<&str>, index: usize) -> Option<Lesson> {
        lazy_static! {
            static ref GROUP_RE: Regex =
                Regex::new(r"\d(CIFS|BABM|BIS|CL|ECwF|Fin|BM(Fin|Mar))\d+").unwrap();
            static ref KILL_BRACKETS_RE: Regex = Regex::new(r"\s?\(\s?\d+\s?\)").unwrap();
        }
        // get rid of strings with just whitespace and strings that have group names
        let slot: Vec<&str> = slot
            .into_iter()
            .filter(|text| !(text.trim().is_empty() || GROUP_RE.is_match(text)))
            .collect();
        // non-empty slot at this point should look like:
        // ["location", "module name_format_blah", "teacher"]

        if slot.is_empty() {
            None
        } else {
            let (name, format) = Self::process_class(slot[1]);
            let tutor = slot[2].trim().to_string();
            let start = 9.0 + index as f32;
            let location = KILL_BRACKETS_RE.replace(slot[0], "").trim().to_string();

            Some(Lesson {
                name,
                tutor,
                format,
                start,
                length: 1.0,
                location,
            })
        }
    }

    /// Return a tuple with class name as the first & class format as the second item.
    /// The input string is usually written as: "module name_format_extra_info".
    fn process_class(class: &str) -> (String, String) {
        let class: Vec<&str> = class.splitn(2, "_").collect();
        let mut format = String::new();

        let mut name = class[0].trim().to_string();
        if name.starts_with("online / ") {
            format += "online ";
            name = name.strip_prefix("online / ").unwrap().to_string();
        }
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
#[derive(Serialize)]
pub struct TimeTable {
    pub monday: Day,
    pub tuesday: Day,
    pub wednesday: Day,
    pub thursday: Day,
    pub friday: Day,
    pub saturday: Day,
}

impl TimeTable {
    /// Construct a new instance of an empty TimeTable.
    fn new() -> Self {
        TimeTable {
            monday: Vec::new(),
            tuesday: Vec::new(),
            wednesday: Vec::new(),
            thursday: Vec::new(),
            friday: Vec::new(),
            saturday: Vec::new(),
        }
    }

    /// Parse the given HTML to construct a new instance of the TimeTable.
    pub fn parse(table: Html) -> Self {
        let mut timetable = TimeTable::new();

        // first row is excluded as it only has information about the timeslots
        let selector = Selector::parse("div.row.cf:not(:first-of-type)").unwrap();
        let rows = table.select(&selector);

        for (index, row) in rows.enumerate() {
            // first slot is excluded as it only has info on weekday
            let selector = Selector::parse("div.col:not(:first-of-type) .innerbox").unwrap();
            let slots = row
                .select(&selector)
                .map(|el| el.text().collect::<Vec<&str>>())
                .collect::<Vec<_>>();
            let day = Self::get_day(slots);

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

    /// Parse each slot and return a list of lessons for a given day.
    fn get_day(slots: Vec<Vec<&str>>) -> Day {
        let mut day: Vec<Lesson> = Vec::new();
        for (index, slot) in slots.into_iter().enumerate() {
            let lesson = Lesson::new(slot, index);

            if let Some(lesson) = lesson {
                match day.last_mut() {
                    Some(last_lesson) => {
                        if lesson.is_continuation(last_lesson) {
                            last_lesson.prolong();
                        } else {
                            day.push(lesson)
                        }
                    }
                    None => day.push(lesson),
                }
            }
        }

        day
    }
}
