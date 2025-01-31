use std::fs::File;
use std::io::{self, BufReader, BufRead};
use transaction::Transaction;

mod location;
mod transaction;

// in main function
// a. create file variable by passing "./transactions.csv" into the File::open function,
// followed by calling the unwrap method
// b. create reader variable by passing file variable into the BufReader::new function
// c. create mutable transactions variable of type Vec<Transaction> by calling Vec::new
// method
// d. create mutable skipped_lines variable of no explicit type simply calling Vec::new method
// e. run a for loop destructured into arbitrary variables of (idx, line) using the reader variable
// which calls lines method followed by enumerate method
// - if idx equals to 0, continue
// - create line_str variable by using line to call the unwrap method
// - create parsed_transaction variable by passing line_str into
// Transaction::fram_csv_line method
// - match on parsed_transaction
// - if matches on Ok variant, push value within Ok into transactions
// - If matches on Err variant, push the tuple of (idx, value within Err, line_str)
// into skipped_lines
// f. run a for loop by using transactions to call the iter method
// - print every item in transactions
// g. do the same for skipped_lines

fn main() {
  let file = File::open("./transactions.csv").unwrap();
  let reader = BufReader::new(file);

  let mut transactions: Vec<Transaction> = Vec::new();
  let mut skipped_lines = Vec::new();

  for (idx, line) in reader.lines().enumerate() {
    if idx == 0 {
      continue;
    }

    let line_str = line.unwrap();
    let parsed_transaction = Transaction::from_csv_line(&line_str);
    match parsed_transaction {
      Ok(transaction) => transactions.push(transaction),
      Err(error_string) => skipped_lines.push((idx, error_string, line_str)),
    }
  }

  for transaction in transactions.iter() {
    println!("{:?}", transaction);
  }

  for skipped_transactions in skipped_lines.iter() {
    println!("{:?}", skipped_transactions);
  }

  // Bonus (Distinction)
  // Utilize HashMap to keep track of the total invested amount per continent and print the result out for each continent
  // - Hint: You would need to convert the continent to String to store as keys
  // - Create a function that takes in a reference slice of transactions and a reference of Continent, and filters rows by the Continent. Print only transactions with European companies
  // - Hint: You would need to utilise lifetimes, iterators, and filter function

}
