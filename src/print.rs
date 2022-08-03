use tabled::{object::*, width::*, *};

pub fn print<T: Tabled>(caption: &str, iter: impl IntoIterator<Item = T>) {
    println!(
        "{}",
        Table::new(iter)
            .with(Width::increase(100))
            .with(Header(caption))
            .with(Style::modern())
            .with(
                Modify::new(Segment::all())
                    .with(Alignment::center())
                    .with(Alignment::center_vertical())
            )
            .with(Modify::new(Rows::new(1..)).with(Width::wrap(14).keep_words()))
            .to_string()
    );
}
