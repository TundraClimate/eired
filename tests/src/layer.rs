use eired_display::{Annotate, Layer, Span};

#[test]
fn push_write_begin() {
    let mut layer = Layer::default();

    layer.push_span_write(Span::from("Hi, World!").annotate((0, 0)));
    layer.push_span_write(Span::from("Hello").annotate((0, 0)));

    assert!(layer.inner().len() == 2);
    assert_eq!(layer.inner()[0].inner(), &Span::from("orld!"));
    assert_eq!(layer.inner()[1].inner(), &Span::from("Hello"));
}

#[test]
fn push_write_include() {
    let mut layer = Layer::default();

    layer.push_span_write(Span::from("Hello, World!").annotate((0, 0)));
    layer.push_span_write(Span::from("!").annotate((5, 0)));

    assert!(layer.inner().len() == 3);
    assert_eq!(layer.inner()[0], Span::from("Hello").annotate((0, 0)));
    assert_eq!(layer.inner()[1], Span::from(" World!").annotate((6, 0)));
    assert_eq!(layer.inner()[2], Span::from("!").annotate((5, 0)));
}

#[test]
fn push_write_end() {
    let mut layer = Layer::default();

    layer.push_span_write(Span::from("Hello! World!").annotate((0, 0)));
    layer.push_span_write(Span::from(", World!!").annotate((5, 0)));

    assert!(layer.inner().len() == 2);
    assert_eq!(layer.inner()[0], Span::from("Hello").annotate((0, 0)));
    assert_eq!(layer.inner()[1], Span::from(", World!!").annotate((5, 0)));
}

#[test]
fn push_fixed_begin() {
    let mut layer = Layer::default();

    layer.push_span_fixed(Span::from("Hello,").annotate((0, 0)));
    layer.push_span_fixed(Span::from("Sorry, World!").annotate((0, 0)));

    assert!(layer.inner().len() == 2);
    assert_eq!(layer.inner()[0], Span::from("Hello,").annotate((0, 0)));
    assert_eq!(layer.inner()[1], Span::from(" World!").annotate((6, 0)));
}

#[test]
fn push_fixed_include() {
    let mut layer = Layer::default();

    layer.push_span_fixed(Span::from("llo, W").annotate((2, 0)));
    layer.push_span_fixed(Span::from("Hello, World!").annotate((0, 0)));

    assert!(layer.inner().len() == 3);
    assert_eq!(layer.inner()[0], Span::from("llo, W").annotate((2, 0)));
    assert_eq!(layer.inner()[1], Span::from("He").annotate((0, 0)));
    assert_eq!(layer.inner()[2], Span::from("orld!").annotate((8, 0)));
}

#[test]
fn push_fixed_end() {
    let mut layer = Layer::default();

    layer.push_span_fixed(Span::from("World!").annotate((7, 0)));
    layer.push_span_fixed(Span::from("Hello, What?").annotate((0, 0)));

    assert!(layer.inner().len() == 2);
    assert_eq!(layer.inner()[0], Span::from("World!").annotate((7, 0)));
    assert_eq!(layer.inner()[1], Span::from("Hello, ").annotate((0, 0)));
}

#[test]
fn push_fixed_ignore() {
    let mut layer = Layer::default();

    layer.push_span_fixed(Span::from("Hello, World!").annotate((0, 0)));
    layer.push_span_fixed(Span::from("Hello, What?").annotate((0, 0)));

    assert!(layer.inner().len() == 1);
    assert_eq!(
        layer.inner()[0],
        Span::from("Hello, World!").annotate((0, 0))
    );
}

#[test]
fn push_only_valid() {
    let mut layer = Layer::default();

    layer.push_span_only_valid(Span::from("Hello, World!").annotate((0, 0)));
    layer.push_span_only_valid(Span::from("Hello, World!!").annotate((0, 0)));

    assert!(layer.inner().len() == 1);
    assert_eq!(
        layer.inner()[0],
        Span::from("Hello, World!").annotate((0, 0))
    );
}

#[test]
fn overlap_layer() {
    let mut layer1 = Layer::default();

    layer1.push_span_write(Span::from("Hello, World!").annotate((0, 0)));

    let mut layer2 = Layer::default();

    layer2.push_span_write(Span::from("________").annotate((2, 0)));

    let layer = layer1.overlap((0, 0), layer2.annotate((0, 0)));

    assert!(layer.inner().inner().len() == 3);
    assert_eq!(layer.inner().inner()[0], Span::from("He").annotate((0, 0)));
    assert_eq!(
        layer.inner().inner()[1],
        Span::from("ld!").annotate((10, 0))
    );
    assert_eq!(
        layer.inner().inner()[2],
        Span::from("________").annotate((2, 0))
    );
}

#[test]
fn annot_offset() {
    let mut layer1 = Layer::default();

    layer1.push_span_write(Span::from("Hello, World!").annotate((0, 0)));

    let mut layer2 = Layer::default();

    layer2.push_span_write(Span::from("Hi, World!").annotate((0, 0)));

    let layer = layer1.overlap((0, 0), layer2.annotate((0, 2)));

    assert!(layer.inner().inner().len() == 2);
    assert_eq!(
        layer.inner().inner()[0],
        Span::from("Hello, World!").annotate((0, 0))
    );
    assert_eq!(
        layer.inner().inner()[1],
        Span::from("Hi, World!").annotate((0, 2))
    );
}

#[test]
fn offset_addr() {
    let mut layer = Layer::default();

    layer.push_span_write(Span::from("Hello, World!").annotate((0, 0)));
    layer.add_offset((5, 1));

    assert!(layer.inner().len() == 1);
    assert_eq!(
        layer.inner()[0],
        Span::from("Hello, World!").annotate((5, 1))
    );
}
