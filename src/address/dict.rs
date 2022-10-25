use super::data::DICT;
use crate::pick_one;
use std::collections::binary_heap::Iter;
use std::collections::HashMap;

#[derive(Clone)]
struct Item {
    id: String,
    pid: Option<String>,
    value: String,
    children: Option<Vec<HashMap<Option<String>, Item>>>,
}
fn get_tree() {
    let dict = DICT.map(|(key, value)| {
        let mut pid: Option<String> = None;
        if key.ends_with("0000") {
            pid = None;
        } else if key.ends_with("00") {
            pid = key.get(..2).map(|x| format!("{}0000", x));
        } else {
            pid = key.get(..4).map(|x| format!("{}0000", x));
        }
        Item {
            id: key.to_string(),
            pid,
            value: value.to_string(),
            children: None,
        }
    });
    let mut hash_map = HashMap::new();
    dict.iter().for_each(|x| {
        hash_map.insert(x.clone().pid, x);
    });
}

fn get_provinces() -> Vec<(&'static str, &'static str)> {
    DICT.into_iter()
        .filter(|(_address, code)| code.ends_with("0000"))
        .collect::<Vec<(&str, &str)>>()
}

fn random_province() -> &'static str {
    let provinces = get_provinces();
    pick_one(&provinces).1
}
