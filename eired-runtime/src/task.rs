use crossbeam::channel::{RecvError, Sender};

use eired_display::VTerm;

pub enum RuntimeTask {
    UpdateBuffer(VTerm),
    ClearBuffer,
    ShowCursor,
    HideCursor,
    MoveCursor(u16, u16),
    Sync(Sender<()>),
    Close,
}

impl RuntimeTask {
    pub(crate) fn eval(task: Result<RuntimeTask, RecvError>, ctx: TaskContext) {
        match task {
            Ok(RuntimeTask::UpdateBuffer(vterm)) => {
                *ctx.buffer = Some(vterm);
            }
            Ok(RuntimeTask::ClearBuffer) => {
                *ctx.buffer = None;
            }
            Ok(RuntimeTask::ShowCursor) => {
                *ctx.cursor_vis = true;
            }
            Ok(RuntimeTask::HideCursor) => {
                *ctx.cursor_vis = false;
            }
            Ok(RuntimeTask::MoveCursor(x, y)) => {
                *ctx.cursor = Some((x, y));
            }
            Ok(RuntimeTask::Sync(tx)) => {
                tx.send(()).ok();
            }
            Ok(RuntimeTask::Close) | Err(_) => {
                *ctx.running = false;
            }
        }
    }
}

pub(crate) struct TaskContext<'a> {
    pub(crate) buffer: &'a mut Option<VTerm>,
    pub(crate) cursor: &'a mut Option<(u16, u16)>,
    pub(crate) cursor_vis: &'a mut bool,
    pub(crate) running: &'a mut bool,
}
