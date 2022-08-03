use tabled::{Style, Table, Tabled};

pub fn print<T: Tabled>(caption: &str, iter: impl IntoIterator<Item = T>) {
    println!("{}", caption);
    println!("{}", Table::new(iter).with(Style::modern()).to_string());
}
