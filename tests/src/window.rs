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
                    Cell::new('.'),
                    Cell::new('.'),
                    Cell::new('.'),
                    Cell::new('.'),
                    Cell::new('.'),
                    Cell::new('.'),
                    Cell::new('.'),
                    Cell::new('.'),
                    Cell::new('.'),
                ],
            )
            .annotate((0, 0)),
            View::new(
                3,
                3,
                vec![
                    Cell::new('O'),
                    Cell::new('O'),
                    Cell::new('O'),
                    Cell::new('O'),
                    Cell::new('O'),
                    Cell::new('O'),
                    Cell::new('O'),
                    Cell::new('O'),
                    Cell::new('O'),
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
            Cell::new('.'),
            Cell::new('.'),
            Cell::new('.'),
            Cell::new('.'),
            Cell::new('O'),
            Cell::new('O'),
            Cell::new('.'),
            Cell::new('O'),
            Cell::new('O'),
        ]
    )
}
