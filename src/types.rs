pub struct Lesson {
    pub name: String,
    pub tutor: String,
    pub types: String,
    pub start: f32,
    pub length: f32,
    pub location: String,
}

pub type Day = Vec<Lesson>;

pub struct Group {
    pub monday: Day,
    pub tuesday: Day,
    pub wednesday: Day,
    pub thursday: Day,
    pub friday: Day,
    pub saturday: Day,
}

// (Group, Code)
pub type Code = (String, String);
