use tabled::{Table, Tabled};

/// Prints a table with rounded borders.
pub fn print<T: Tabled>(rows: impl IntoIterator<Item = T>) {
    let mut table = Table::new(rows);
    table.with(tabled::settings::Style::rounded());
    println!("{table}");
}
