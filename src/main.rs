use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};

#[derive(Serialize, Deserialize, Debug)]
struct BudgetItem {
    name: String,
    amount: f64,
    category: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("database.json").expect("Unable to open file");
    let reader = BufReader::new(file);

    let budget_items: Vec<BudgetItem> = serde_json::from_reader(reader)?;

    for item in budget_items {
        println!("{:?}", item);
    }

    Ok(())
}
