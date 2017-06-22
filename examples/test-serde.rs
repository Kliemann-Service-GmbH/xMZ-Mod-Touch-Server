#[macro_use] extern crate serde_derive;
extern crate serde_json;

use std::cell::{Cell, RefCell};
use std::sync::{Mutex, RwLock};


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct TestSerde {
    num: i32,
    cell: Cell<i32>,
    ref_cell: RefCell<i32>,
    mutex: Mutex<i32>,
    rwlock: RwLock<i32>,
    // #[serde(skip_deserializing)]
    // arc: Arc<i32>,
}

impl TestSerde {
    fn new() -> Self {
        TestSerde {
            num: 0,
            cell: Cell::new(0),
            ref_cell: RefCell::new(0),
            mutex: Mutex::new(0),
            rwlock: RwLock::new(0),
            // arc: Arc::new(0),
        }
    }
}

impl Default for TestSerde {
    fn default() -> TestSerde {
        TestSerde::new()
    }
}


fn main() {
    let test_serde = TestSerde::new();
    println!("{:#?}", test_serde);

    let json = serde_json::to_string_pretty(&test_serde).unwrap();
    println!("{:#?}", json);

    let test_serde_from_json: TestSerde = serde_json::from_str(&json).unwrap();
    println!("{:#?}", test_serde_from_json);

}
