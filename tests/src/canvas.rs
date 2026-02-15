use eired_display::{Annot, Canvas, Cell, Layer, Span};

#[test]
fn apply_layer() {
    let mut canvas = Canvas::default();
    let mut layer1 = Layer::default();
    let mut layer2 = Layer::default();

    layer1.push_span_write(Annot::new((0, 0), 1, 1, Span::from("A")));
    layer2.push_span_write(Annot::new((0, 1), 1, 1, Span::from("B")));

    canvas.overlap_layer(layer1);
    canvas.overlap_layer(layer2);

    assert!(canvas.inner_vec().len() == 2);
    assert_eq!(canvas.inner_vec()[0].0, &0);
    assert_eq!(canvas.inner_vec()[1].0, &1);
    assert_eq!(canvas.inner_vec()[0].1.inner()[0].inner(), &Span::from("A"));
    assert_eq!(canvas.inner_vec()[1].1.inner()[0].inner(), &Span::from("B"));
}

#[test]
fn merge_layer() {
    let mut canvas = Canvas::default();
    let mut layer1 = Layer::default();
    let mut layer2 = Layer::default();

    layer1.push_span_write(Annot::new((3, 0), 10, 1, Span::from("Hi, World!")));
    layer2.push_span_write(Annot::new((0, 0), 6, 1, Span::from("Hello,")));

    canvas.insert_or_merge(0, layer1);
    canvas.insert_or_merge(0, layer2);

    assert!(canvas.inner_vec().len() == 1);
    assert_eq!(
        canvas.inner_vec()[0].1.inner()[0].inner(),
        &Span::from(" World!")
    );
    assert_eq!(
        canvas.inner_vec()[0].1.inner()[1].inner(),
        &Span::from("Hello,")
    );
}

#[test]
fn create_view() {
    let mut canvas = Canvas::default();
    let mut layer = Layer::default();

    layer.push_span_write(Annot::new((0, 0), 3, 1, Span::from("...")));
    layer.push_span_write(Annot::new((0, 1), 3, 1, Span::from("...")));
    layer.push_span_write(Annot::new((0, 2), 3, 1, Span::from("...")));

    canvas.overlap_layer(layer);

    let view = canvas.create_view();

    assert!(view.len() == 3 * 3);

    for cell in view.iter().flatten() {
        assert_eq!(cell, &Cell::from('.'))
    }
}
