use eired::TuiEngine;
use eired_display::Annotate;
use eired_display::Span;
use eired_display::View;

fn main() {
    let engine = TuiEngine::default();

    engine.run(|frame| {
        frame.overlap(View::new(3, 3, Span::from("AAABBBCCC").to_vec()).annotate((1, 1)));

        frame.show_cursor();
        frame.cursor_move_to(4, 3);

        frame.update_frame()?;

        std::thread::sleep(std::time::Duration::from_secs(3));

        frame.overlap(View::new(3, 3, Span::from("AAABBBCCC").to_vec()).annotate((1, 1)));

        frame.hide_cursor();

        frame.update_frame()?;

        std::thread::sleep(std::time::Duration::from_secs(2));

        Ok(())
    });
}
