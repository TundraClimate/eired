use eired_display::{Annotate, Cell, DrawableSpan, View, Window};

#[test]
fn convert_to_cmds() {
    let window = Window::from_views(
        3,
        3,
        vec![
            View::new(
                3,
                3,
                vec![
                    Some(Cell::new('.')),
                    Some(Cell::new('.')),
                    Some(Cell::new('.')),
                    Some(Cell::new('.')),
                    Some(Cell::new('.')),
                    Some(Cell::new('.')),
                    Some(Cell::new('.')),
                    Some(Cell::new('.')),
                    Some(Cell::new('.')),
                ],
            )
            .annotate((0, 0)),
            View::new(
                3,
                3,
                vec![
                    Some(Cell::new('O')),
                    Some(Cell::new('O')),
                    Some(Cell::new('O')),
                    Some(Cell::new('O')),
                    Some(Cell::new('O')),
                    Some(Cell::new('O')),
                    Some(Cell::new('O')),
                    Some(Cell::new('O')),
                    Some(Cell::new('O')),
                ],
            )
            .annotate((1, 1)),
        ],
    );

    let res = eired_display::create_virtual_terminal(window.annotate((0, 0)));

    let res = eired_display::convert_to_spans(res);

    assert!(res.len() == 3);
    assert_eq!(
        res[0],
        DrawableSpan::new((0, 0), [Cell::new('.'), Cell::new('.'), Cell::new('.'),])
    );
    assert_eq!(
        res[1],
        DrawableSpan::new((0, 1), [Cell::new('.'), Cell::new('O'), Cell::new('O'),])
    );
    assert_eq!(
        res[2],
        DrawableSpan::new((0, 2), [Cell::new('.'), Cell::new('O'), Cell::new('O'),])
    );
}

#[test]
fn convert_to_cmds_with_offset() {
    let window = Window::from_views(
        3,
        3,
        vec![
            View::new(
                3,
                3,
                vec![
                    Some(Cell::new('.')),
                    Some(Cell::new('.')),
                    Some(Cell::new('.')),
                    Some(Cell::new('.')),
                    Some(Cell::new('.')),
                    Some(Cell::new('.')),
                    Some(Cell::new('.')),
                    Some(Cell::new('.')),
                    Some(Cell::new('.')),
                ],
            )
            .annotate((0, 0)),
            View::new(
                3,
                3,
                vec![
                    Some(Cell::new('O')),
                    Some(Cell::new('O')),
                    Some(Cell::new('O')),
                    Some(Cell::new('O')),
                    Some(Cell::new('O')),
                    Some(Cell::new('O')),
                    Some(Cell::new('O')),
                    Some(Cell::new('O')),
                    Some(Cell::new('O')),
                ],
            )
            .annotate((1, 1)),
        ],
    );

    let res = eired_display::create_virtual_terminal(window.annotate((1, 1)));

    let res = eired_display::convert_to_spans(res);

    assert!(res.len() == 3);
    assert_eq!(
        res[0],
        DrawableSpan::new((1, 1), [Cell::new('.'), Cell::new('.'), Cell::new('.'),])
    );
    assert_eq!(
        res[1],
        DrawableSpan::new((1, 2), [Cell::new('.'), Cell::new('O'), Cell::new('O'),])
    );
    assert_eq!(
        res[2],
        DrawableSpan::new((1, 3), [Cell::new('.'), Cell::new('O'), Cell::new('O'),])
    );
}
