struct KooItem {
      value: String,
       done: bool,
}

pub struct KooList {
    list: Vec<KooItem>,
}

impl KooList {
    pub fn add(&self, value: &str) {
        // some code
    }

    pub fn delete(&self, item_id: i32) {
        // some code
    }
}
