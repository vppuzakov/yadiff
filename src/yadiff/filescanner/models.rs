pub enum Categories {
    DIRECTORY,
    FILE,
}

pub struct Resource {
    pub name: String,
    pub path: String,
    pub category: Categories,
}
