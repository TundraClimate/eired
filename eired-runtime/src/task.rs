use crossbeam::channel::{RecvError, Sender};

use eired_display::Point;

use crate::Canvas;

pub enum RuntimeTask {
    UpdateBuffer(Canvas),
    ClearBuffer,
    ShowCursor,
    HideCursor,
    MoveCursor(Point),
    Sync(Sender<()>),
    Close,
}

impl RuntimeTask {
    pub(crate) fn eval(task: Result<RuntimeTask, RecvError>, ctx: TaskContext) {
        match task {
            Ok(RuntimeTask::UpdateBuffer(canvas)) => {
                *ctx.buffer = Some(canvas);
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
            Ok(RuntimeTask::MoveCursor(at)) => {
                *ctx.cursor = Some(at);
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
    pub(crate) buffer: &'a mut Option<Canvas>,
    pub(crate) cursor: &'a mut Option<Point>,
    pub(crate) cursor_vis: &'a mut bool,
    pub(crate) running: &'a mut bool,
}
