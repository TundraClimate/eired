use std::io::{self, Write};
use std::sync::{Arc, Mutex};

use crossbeam::channel;

use eired_display::{Span, VTerm};
use eired_runtime::RenderRuntime;
use eired_runtime::config::{ConfigBuilder, RuntimeConfig};
use eired_runtime::renderer::{Diff, Renderer};
use eired_runtime::task::RuntimeTask;

struct Dummyout;

impl Write for Dummyout {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        unimplemented!();
    }

    fn flush(&mut self) -> io::Result<()> {
        unimplemented!();
    }
}

#[derive(Default)]
struct DummyRenderer {
    contents: Arc<Mutex<Vec<String>>>,
}

impl Renderer<Dummyout> for DummyRenderer {
    fn render(&mut self, _config: &RuntimeConfig, diff: Diff) -> io::Result<()> {
        let cmds = eired_display::convert_to_draws((0, 0), diff.into_vec());

        let contents = &mut *self.contents.lock().unwrap();

        for cmd in cmds {
            contents.push(cmd.raw_content());
        }

        Ok(())
    }

    fn store(&mut self, _config: &RuntimeConfig) -> io::Result<()> {
        Ok(())
    }

    fn restore(&mut self, _config: &RuntimeConfig) -> io::Result<()> {
        Ok(())
    }
}

fn make_from_lines(lines: &[&str]) -> VTerm {
    let cells = lines
        .iter()
        .map(|l| {
            let span = Span::from(*l).to_vec();

            span.into_iter().fold(vec![], |mut acc, c| {
                acc.push(c);

                acc
            })
        })
        .collect::<Vec<_>>()
        .concat();

    let width = lines[0].len() as u16;
    let height = lines.len() as u16;

    VTerm::new(width, height, &cells)
}

#[test]
fn renderer_update() {
    let config = ConfigBuilder::default().no_tick().build();
    let renderer = DummyRenderer::default();
    let contents = renderer.contents.clone();
    let (runtime, tx) = RenderRuntime::new(config, renderer);
    let (sync_tx, sync_rx) = channel::bounded(10);

    let handle = runtime.spawn();

    assert_eq!(contents.lock().unwrap().len(), 0);

    tx.send(RuntimeTask::UpdateBuffer(make_from_lines(&[
        "XXX   XXX",
        "   OOO   ",
        "XXX   XXX",
    ])))
    .ok();

    tx.send(RuntimeTask::Sync(sync_tx.clone())).ok();

    sync_rx.recv().ok();

    assert_eq!(contents.lock().unwrap().len(), 3);
    assert_eq!(contents.lock().unwrap()[0], "XXX   XXX".to_string());
    assert_eq!(contents.lock().unwrap()[1], "   OOO   ".to_string());
    assert_eq!(contents.lock().unwrap()[2], "XXX   XXX".to_string());

    tx.send(RuntimeTask::UpdateBuffer(make_from_lines(&[
        "XXX   XXX",
        "   XXX   ",
        "XXX   XXX",
    ])))
    .ok();

    tx.send(RuntimeTask::Sync(sync_tx.clone())).ok();

    sync_rx.recv().ok();

    assert_eq!(contents.lock().unwrap().len(), 4);
    assert_eq!(contents.lock().unwrap()[3], "XXX".to_string());

    tx.send(RuntimeTask::Close).ok();

    handle.join().ok();
}
