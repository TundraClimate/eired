use eired::TuiEngine;
use eired::terminal::Annotate;
use eired::widget::Span;

fn main() {
    let engine = TuiEngine::default();

    engine.run(|frame| {
        frame.draw(Span::from("AAA").annotate((1, 1)));
        frame.draw(Span::from("BBB").annotate((1, 2)));
        frame.draw(Span::from("CCC").annotate((1, 3)));

        frame.show_cursor();
        frame.cursor_move_to((4, 3));

        frame.update_frame()?;

        std::thread::sleep(std::time::Duration::from_secs(3));

        frame.draw(Span::from("AAA").annotate((1, 1)));
        frame.draw(Span::from("BBB").annotate((1, 2)));
        frame.draw(Span::from("CCC").annotate((1, 3)));

        frame.hide_cursor();

        frame.update_frame()?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        Ok(())
    });
}
