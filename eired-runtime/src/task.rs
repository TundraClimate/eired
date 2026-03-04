use crossbeam::channel::RecvError;

use eired_display::VTerm;

pub enum RuntimeTask {
    UpdateBuffer(VTerm),
    Close,
}

impl RuntimeTask {
    pub(crate) fn eval(task: Result<RuntimeTask, RecvError>, ctx: TaskContext) {
        match task {
            Ok(RuntimeTask::UpdateBuffer(vterm)) => {
                *ctx.buffer = Some(vterm);
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
