use std::sync::Mutex;

static STORAGE: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub fn add_item(item: String) {
    let mut vec = STORAGE.lock().unwrap();
    vec.push(item);
}

pub fn get_item(item: String) -> bool {
    let vec = STORAGE.lock().unwrap();
    vec.contains(&item)
}

pub fn get_all_items() -> Vec<String> {
    let vec = STORAGE.lock().unwrap();
    vec.clone()
}