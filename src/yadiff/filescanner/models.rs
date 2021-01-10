pub enum Categories {
    DIRECTORY,
    FILE,
}

pub struct Resource {
    pub name: String,
    pub path: String,
    pub category: Categories,
}

impl Resource {
    pub fn fixpath(&mut self, parent: &String) {
        self.path = str::replace(&self.path, parent, "");
    }
}
