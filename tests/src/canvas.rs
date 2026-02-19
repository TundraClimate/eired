use eired_display::{Annotate, Canvas, Cell, Layer, Span};

#[test]
fn apply_layer() {
    let mut canvas = Canvas::default();
    let mut layer1 = Layer::default();
    let mut layer2 = Layer::default();

    layer1.push_span_write(Span::from("A").annotate((0, 0)));
    layer2.push_span_write(Span::from("B").annotate((0, 1)));

    canvas.overlap_layer(layer1.annotate((0, 0)));
    canvas.overlap_layer(layer2.annotate((0, 0)));

    assert!(canvas.inner_vec().len() == 2);
    assert_eq!(canvas.inner_vec()[0].0, &0);
    assert_eq!(canvas.inner_vec()[1].0, &1);
    assert_eq!(
        canvas.inner_vec()[0].1.inner().inner()[0].inner(),
        &Span::from("A")
    );
    assert_eq!(
        canvas.inner_vec()[1].1.inner().inner()[0].inner(),
        &Span::from("B")
    );
}

#[test]
fn merge_layer() {
    let mut canvas = Canvas::default();
    let mut layer1 = Layer::default();
    let mut layer2 = Layer::default();

    layer1.push_span_write(Span::from("Hi, World!").annotate((3, 0)));
    layer2.push_span_write(Span::from("Hello,").annotate((0, 0)));

    canvas.insert_or_merge(0, layer1.annotate((0, 0)));
    canvas.insert_or_merge(0, layer2.annotate((0, 0)));

    assert!(canvas.inner_vec().len() == 1);
    assert_eq!(
        canvas.inner_vec()[0].1.inner().inner()[0].inner(),
        &Span::from(" World!")
    );
    assert_eq!(
        canvas.inner_vec()[0].1.inner().inner()[1].inner(),
        &Span::from("Hello,")
    );
}

#[test]
fn create_view() {
    let mut canvas = Canvas::default();
    let mut layer = Layer::default();

    layer.push_span_write(Span::from("...").annotate((0, 0)));
    layer.push_span_write(Span::from("...").annotate((0, 1)));
    layer.push_span_write(Span::from("...").annotate((0, 2)));

    canvas.overlap_layer(layer.annotate((0, 0)));

    let view = canvas.create_view();

    assert!(view.len() == 3 * 3);

    for cell in view.iter().flatten() {
        assert_eq!(cell, &Cell::from('.'))
    }
}

#[test]
fn create_view_with_multi_layer() {
    let mut canvas = Canvas::default();

    let mut layer1 = Layer::default();

    layer1.push_span_write(Span::from("...").annotate((0, 0)));
    layer1.push_span_write(Span::from("...").annotate((0, 1)));
    layer1.push_span_write(Span::from("...").annotate((0, 2)));

    canvas.overlap_layer(layer1.annotate((0, 0)));

    let mut layer2 = Layer::default();

    layer2.push_span_write(Span::from("OOO").annotate((0, 0)));
    layer2.push_span_write(Span::from("OOO").annotate((0, 1)));
    layer2.push_span_write(Span::from("OOO").annotate((0, 2)));

    canvas.overlap_layer(layer2.annotate((1, 2)));

    let view = canvas.create_view();

    let expected = [
        Some(Cell::from('.')),
        Some(Cell::from('.')),
        Some(Cell::from('.')),
        None,
        Some(Cell::from('.')),
        Some(Cell::from('.')),
        Some(Cell::from('.')),
        None,
        Some(Cell::from('.')),
        Some(Cell::from('O')),
        Some(Cell::from('O')),
        Some(Cell::from('O')),
        None,
        Some(Cell::from('O')),
        Some(Cell::from('O')),
        Some(Cell::from('O')),
        None,
        Some(Cell::from('O')),
        Some(Cell::from('O')),
        Some(Cell::from('O')),
    ];

    assert!(view.len() == expected.len());

    assert_eq!(
        view.iter().collect::<Vec<_>>(),
        expected.iter().collect::<Vec<_>>()
    );
}
