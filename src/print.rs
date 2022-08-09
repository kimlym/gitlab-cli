use tabled::{object::*, width::*, *};

pub fn print<T: Tabled>(caption: &str, iter: impl IntoIterator<Item = T>) {
    let mut list: Vec<T> = vec![];
    for item in iter {
        list.push(item);
    }
    let item_field_num = list.get(0).unwrap().fields().len();

    let mut table = Table::new(list)
        .with(Width::increase(100))
        .with(Header(caption))
        .with(Style::modern())
        .with(
            Modify::new(Segment::all())
                .with(Alignment::center())
                .with(Alignment::center_vertical()),
        );

    if item_field_num > 1 {
        table = table.with(Modify::new(Columns::new(2..)).with(Width::wrap(14).keep_words()));
    }

    println!("{}", table.to_string());
}

mod test {
    use std::fmt::Display;

    use super::*;
    use terminal_link::Link;

    #[derive(Tabled)]
    struct Test<'a> {
        id: i32,
        link: MyLink<'a>,
    }

    #[derive(Tabled)]
    struct MyLink<'a>(Link<'a>);

    impl Display for MyLink<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[test]
    fn should_print_one_column_with_link_work() {
        let a = "\u{1b}]8;;http://www.google.com\u{1b}\\google\u{1b}]8;;\u{1b}\\";
        print("Single column", vec![a]);
    }

    #[test]
    fn should_print_multi_columns_with_link_work() {
        let test = Test {
            link: MyLink(Link::new("123", "http://www.google.com")),
            id: 123,
        };

        print("Two columns", vec![test]);
    }
}
