use eired::TuiEngine;
use eired::terminal::Annotate;
use eired::widget::{Layer, Span};

fn main() {
    let engine = TuiEngine::default();

    engine.run(|frame| {
        frame.draw(
            Layer::with_size((3, 3), Span::from("AAABBBCCC"))
                .unwrap()
                .annotate((1, 1)),
        );

        frame.show_cursor();
        frame.cursor_move_to((4, 3));

        frame.update_frame()?;

        std::thread::sleep(std::time::Duration::from_secs(3));

        frame.draw(
            Layer::with_size((3, 3), Span::from("AAABBBCCC"))
                .unwrap()
                .annotate((1, 1)),
        );

        frame.hide_cursor();

        frame.update_frame()?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        Ok(())
    });
}
