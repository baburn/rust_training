mod cache;

use cache::MemoryCache;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let cache = MemoryCache::new();

    cache.insert("foo", "bar", 3); // Expires in 3 sec
    cache.insert("baz", "qux", 5); // Expires in 5 sec

    println!("✅ foo: {:?}", cache.get("foo"));
    println!("✅ baz: {:?}", cache.get("baz"));

    println!("⏳ Sleeping for 4 seconds...");
    sleep(Duration::from_secs(4));

    println!("⛔ foo (should be expired): {:?}", cache.get("foo"));
    println!("✅ baz (still valid): {:?}", cache.get("baz"));

    // Drop happens when main ends
}
