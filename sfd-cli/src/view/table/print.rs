use tabled::{
    Table, Tabled,
    settings::{Panel, Style},
};

/// Prints a table with rounded borders.
pub fn print<T: Tabled>(rows: impl IntoIterator<Item = T>, title: Option<&str>) {
    let mut table = Table::new(rows);
    table.with(Style::rounded());
    if let Some(title) = title {
        table.with(Panel::header(title));
    }
    println!("{table}");
}
