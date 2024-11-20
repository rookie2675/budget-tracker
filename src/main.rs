use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, Error},
};

#[derive(Serialize, Deserialize, Debug)]
struct BudgetItem {
    name: String,
    amount: f64,
    category: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let expenses_result = read_file("database/expenses.json");

    if let Ok(expenses) = expenses_result {
        for expense in expenses {
            println!("{:?}", expense);
        }
    } else {
        eprintln!("Failed to read expenses: {}", expenses_result.unwrap_err());
    }

    let incomes_result = read_file("database/incomes.json");

    if let Ok(incomes) = incomes_result {
        for income in incomes {
            println!("{:?}", income);
        }
    } else {
        eprintln!("Failed to read incomes: {}", incomes_result.unwrap_err());
    }

    Ok(())
}

fn read_file(file_name: &str) -> Result<Vec<BudgetItem>, Error> {
    let file = File::open(&file_name)?;
    let reader = BufReader::new(file);
    let budget_items: Vec<BudgetItem> = serde_json::from_reader(reader)?;
    Ok(budget_items)
}
