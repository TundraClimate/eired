use crossterm::style::Color;
use eired::TuiEngine;
use eired_display::Annotate;
use eired_display::Cell;
use eired_display::Span;
use eired_display::View;

fn main() {
    let engine = TuiEngine::default();

    engine.run(|frame| {
        frame.overlap(
            View::new(
                frame.width(),
                frame.height(),
                vec![Cell::new(' '); (frame.width() * frame.height()) as usize],
            )
            .annotate((0, 0)),
        );

        frame.overlap(
            View::new(
                frame.width(),
                1,
                vec![Cell::new_bg(' ', Color::DarkGrey); frame.width() as usize],
            )
            .annotate((0, 0)),
        );

        frame.overlap(
            View::new(
                6,
                1,
                Span::new_with_color(" Tab1 ", Color::Yellow, Color::Reset).to_vec(),
            )
            .annotate((0, 0)),
        );

        frame.overlap(
            View::new(
                6,
                1,
                Span::new_with_color(" Tab2 ", Color::Yellow, Color::Black).to_vec(),
            )
            .annotate((7, 0)),
        );

        frame.overlap(
            View::new(
                6,
                1,
                Span::new_with_color(" Tab3 ", Color::Yellow, Color::Black).to_vec(),
            )
            .annotate((14, 0)),
        );

        frame.update_frame()?;

        std::thread::sleep(std::time::Duration::from_secs(5));

        Ok(())
    });
}
