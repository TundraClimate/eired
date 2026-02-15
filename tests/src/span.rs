use crossterm::style::Color;
use eired_display::{Cell, Span};

#[test]
fn check_bg_all() {
    let span = Span::new_with_bg("Hello, World!", Color::Red);

    for i in 0..span.len() as usize {
        assert!(span.get(i).is_some_and(|s| s.bg == Color::Red));
    }
}

#[test]
fn check_fg_all() {
    let span = Span::new_with_fg("Hello, World!", Color::Red);

    for i in 0..span.len() as usize {
        assert!(span.get(i).is_some_and(|s| s.fg == Color::Red));
    }
}

#[test]
fn replace_at() {
    let mut span = Span::new_with_fg("Hello, World!", Color::Red);

    span.replace_at(5, Cell::new_fg(',', Color::Blue));

    let mut new_span = Span::default();

    for c in "Hello, World!".chars() {
        if c == ',' {
            new_span.push_back(Cell::new_fg(c, Color::Blue));
        } else {
            new_span.push_back(Cell::new_fg(c, Color::Red));
        }
    }

    assert_eq!(span, new_span);
}

#[test]
fn init_by() {
    let span1 = Span::new_with_bg("A", Color::Red);
    let span2 = Span::new_with_fg("A", Color::Red);
    let span3 = Span::from("A");
    let span4 = Span::from("A".to_string());
    let span5 = Span::from_iter(vec![Cell::new('A')]);

    assert!(!span1.is_empty());
    assert!(!span2.is_empty());
    assert!(!span3.is_empty());
    assert!(!span4.is_empty());
    assert!(!span5.is_empty());
}

#[test]
fn truncate_front() {
    let mut span = Span::new_with_fg("Hello, World!", Color::Red);
    let new_span = Span::new_with_fg("World!", Color::Red);

    span.truncate_front(7);

    assert_eq!(span, new_span);
}

#[test]
fn truncate_back() {
    let mut span = Span::new_with_fg("Hello, World!", Color::Red);
    let new_span = Span::new_with_fg("Hello,", Color::Red);

    span.truncate_back(7);

    assert_eq!(span, new_span);
}

#[test]
fn split() {
    let span = Span::from("Hello, World!");

    let parts = span.split_by(&[5, 7]);

    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0], Some(Span::from("Hello")));
    assert_eq!(parts[1], Some(Span::from(", ")));
    assert_eq!(parts[2], Some(Span::from("World!")));
}
