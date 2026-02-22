use eired_display::{Annotate, Cell, View, Window};

#[test]
fn create_vterm() {
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

    let res = eired_display::create_virtual_terminal(window.annotate((0, 0))).into_inner();

    assert!(res.len() == 9);
    assert_eq!(
        res.to_vec(),
        vec![
            Some(Cell::new('.')),
            Some(Cell::new('.')),
            Some(Cell::new('.')),
            Some(Cell::new('.')),
            Some(Cell::new('O')),
            Some(Cell::new('O')),
            Some(Cell::new('.')),
            Some(Cell::new('O')),
            Some(Cell::new('O')),
        ]
    )
}
