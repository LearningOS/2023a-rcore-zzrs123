//! Types related to task management

use super::TaskContext;
use crate::config:: MAX_SYSCALL_NUM;
/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,

    /// 标记进程是否已经开始过了
    pub task_doing: bool,

    /// 记录调用了多少次系统调用
    pub task_syscall_times: [u32; MAX_SYSCALL_NUM],
    /// 记录进程开始时间
    pub task_start_time: usize,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
