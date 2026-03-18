use eired_display::{Annotate, Cell, Span, VTerm, View, Window};

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

#[test]
fn construct_window() {
    let mut window = Window::new(4, 4);

    window.overlap(View::new(1, 4, Span::from("OOOO").to_vec()).annotate((0, 0)));
    window.overlap(View::new(1, 4, Span::from("IIII").to_vec()).annotate((3, 0)));
    window.overlap(View::new(3, 2, Span::from("TTTTTT").to_vec()).annotate((1, 1)));

    let vterm = window.into_vterm();

    assert_eq!(
        vterm,
        VTerm::from_lines(
            4,
            4,
            &[
                Span::from("O  I"),
                Span::from("OTTT"),
                Span::from("OTTT"),
                Span::from("O  I")
            ]
        )
    );
}

#[test]
fn into_vterm() {
    let window = Window::from_views(
        5,
        5,
        vec![
            View::new(5, 1, Span::from("IIIII").to_vec()).annotate((1, 1)),
            View::new(6, 1, Span::from("OOOOOO").to_vec()).annotate((0, 3)),
            View::new(1, 1, Span::from("C").to_vec()).annotate((4, 3)),
            View::new(1, 6, Span::from("AAAAAA").to_vec()).annotate((3, 0)),
        ],
    );

    let vterm = window.into_vterm();

    assert_eq!(
        vterm,
        VTerm::from_lines(
            5,
            5,
            &[
                Span::from("   A "),
                Span::from(" IIAI"),
                Span::from("   A "),
                Span::from("OOOAC"),
                Span::from("   A ")
            ]
        )
    );
}
