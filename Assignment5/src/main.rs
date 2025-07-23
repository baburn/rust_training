mod db;
mod models;

use db::KVDatabase;

fn main() {
    let mut db = KVDatabase::new("data.db");

    db.insert("username".into(), "nazeer_babu".into());
    db.insert("role".into(), "developer".into());

    println!("\n📦 Current Records (from heap):");
    for record in db.get_all() {
        println!("{} = {}", record.key, record.value);
    }

    println!("\n🔚 End of main. RAII will handle saving.");
}
