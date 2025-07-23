#[derive(Debug)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

impl KeyValue {
    pub fn new(key: String, value: String) -> Box<Self> {
        let kv = KeyValue { key, value };
        let boxed = Box::new(kv);
        println!(
            "🔍 Boxed KeyValue allocated on heap at: {:p}",
            &*boxed
        );
        boxed
    }
}
