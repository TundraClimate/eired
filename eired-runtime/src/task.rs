use crossbeam::channel::{RecvError, Sender};

use eired_display::VTerm;

pub enum RuntimeTask {
    UpdateBuffer(VTerm),
    ClearBuffer,
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
    pub(crate) running: &'a mut bool,
}
