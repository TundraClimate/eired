use eired_display::{Annot, Layer, Span};

#[test]
fn push_write_begin() {
    let mut layer = Layer::default();

    layer.push_span_write(Annot::new((0, 0), 10, 1, Span::from("Hi, World!")));
    layer.push_span_write(Annot::new((0, 0), 5, 1, Span::from("Hello")));

    assert!(layer.inner().len() == 2);
    assert_eq!(layer.inner()[0].inner(), &Span::from("orld!"));
    assert_eq!(layer.inner()[1].inner(), &Span::from("Hello"));
}

#[test]
fn push_write_include() {
    let mut layer = Layer::default();

    layer.push_span_write(Annot::new((0, 0), 13, 1, Span::from("Hello, World!")));
    layer.push_span_write(Annot::new((5, 0), 1, 1, Span::from("!")));

    assert!(layer.inner().len() == 3);
    assert_eq!(
        layer.inner()[0],
        Annot::new((0, 0), 5, 1, Span::from("Hello"))
    );
    assert_eq!(
        layer.inner()[1],
        Annot::new((6, 0), 7, 1, Span::from(" World!"))
    );
    assert_eq!(layer.inner()[2], Annot::new((5, 0), 1, 1, Span::from("!")));
}

#[test]
fn push_write_end() {
    let mut layer = Layer::default();

    layer.push_span_write(Annot::new((0, 0), 13, 1, Span::from("Hello! World!")));
    layer.push_span_write(Annot::new((5, 0), 9, 1, Span::from(", World!!")));

    assert!(layer.inner().len() == 2);
    assert_eq!(
        layer.inner()[0],
        Annot::new((0, 0), 5, 1, Span::from("Hello"))
    );
    assert_eq!(
        layer.inner()[1],
        Annot::new((5, 0), 9, 1, Span::from(", World!!"))
    );
}

#[test]
fn push_fixed_begin() {
    let mut layer = Layer::default();

    layer.push_span_fixed(Annot::new((0, 0), 6, 1, Span::from("Hello,")));
    layer.push_span_fixed(Annot::new((0, 0), 13, 1, Span::from("Sorry, World!")));

    assert!(layer.inner().len() == 2);
    assert_eq!(
        layer.inner()[0],
        Annot::new((0, 0), 6, 1, Span::from("Hello,"))
    );
    assert_eq!(
        layer.inner()[1],
        Annot::new((6, 0), 7, 1, Span::from(" World!"))
    );
}

#[test]
fn push_fixed_include() {
    let mut layer = Layer::default();

    layer.push_span_fixed(Annot::new((2, 0), 6, 1, Span::from("llo, W")));
    layer.push_span_fixed(Annot::new((0, 0), 13, 1, Span::from("Hello, World!")));

    assert!(layer.inner().len() == 3);
    assert_eq!(
        layer.inner()[0],
        Annot::new((2, 0), 6, 1, Span::from("llo, W"))
    );
    assert_eq!(layer.inner()[1], Annot::new((0, 0), 2, 1, Span::from("He")));
    assert_eq!(
        layer.inner()[2],
        Annot::new((8, 0), 5, 1, Span::from("orld!"))
    );
}

#[test]
fn push_fixed_end() {
    let mut layer = Layer::default();

    layer.push_span_fixed(Annot::new((7, 0), 6, 1, Span::from("World!")));
    layer.push_span_fixed(Annot::new((0, 0), 12, 1, Span::from("Hello, What?")));

    assert!(layer.inner().len() == 2);
    assert_eq!(
        layer.inner()[0],
        Annot::new((7, 0), 6, 1, Span::from("World!"))
    );
    assert_eq!(
        layer.inner()[1],
        Annot::new((0, 0), 7, 1, Span::from("Hello, "))
    );
}

#[test]
fn push_fixed_ignore() {
    let mut layer = Layer::default();

    layer.push_span_fixed(Annot::new((0, 0), 13, 1, Span::from("Hello, World!")));
    layer.push_span_fixed(Annot::new((0, 0), 12, 1, Span::from("Hello, What?")));

    assert!(layer.inner().len() == 1);
    assert_eq!(
        layer.inner()[0],
        Annot::new((0, 0), 13, 1, Span::from("Hello, World!"))
    );
}

#[test]
fn push_only_valid() {
    let mut layer = Layer::default();

    layer.push_span_only_valid(Annot::new((0, 0), 13, 1, Span::from("Hello, World!")));
    layer.push_span_only_valid(Annot::new((0, 0), 14, 1, Span::from("Hello, World!!")));

    assert!(layer.inner().len() == 1);
    assert_eq!(
        layer.inner()[0],
        Annot::new((0, 0), 13, 1, Span::from("Hello, World!"))
    );
}

#[test]
fn overlap_layer() {
    let mut layer1 = Layer::default();

    layer1.push_span_write(Annot::new((0, 0), 13, 1, Span::from("Hello, World!")));

    let mut layer2 = Layer::default();

    layer2.push_span_write(Annot::new((2, 0), 8, 1, Span::from("________")));

    let layer = layer1.overlap(layer2);

    assert!(layer.inner().len() == 3);
    assert_eq!(layer.inner()[0], Annot::new((0, 0), 2, 1, Span::from("He")));
    assert_eq!(
        layer.inner()[1],
        Annot::new((10, 0), 3, 1, Span::from("ld!"))
    );
    assert_eq!(
        layer.inner()[2],
        Annot::new((2, 0), 8, 1, Span::from("________"))
    );
}

#[test]
fn offset_addr() {
    let mut layer = Layer::default();

    layer.push_span_write(Annot::new((0, 0), 13, 1, Span::from("Hello, World!")));
    layer.add_offset((5, 1));

    assert!(layer.inner().len() == 1);
    assert_eq!(
        layer.inner()[0],
        Annot::new((5, 1), 13, 1, Span::from("Hello, World!"))
    );
}
