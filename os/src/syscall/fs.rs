//! File and filesystem-related syscalls
use alloc::sync::Arc;

use crate::fs::{open_file, OpenFlags, Stat, link_file, unlink_file, OSInode, StatMode};
use crate::mm::{translated_byte_buffer, translated_str, UserBuffer};
use crate::task::{current_task, current_user_token};


pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    trace!("kernel:pid[{}] sys_write", current_task().unwrap().pid.0);
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        if !file.writable() {
            return -1;
        }
        let file = file.clone();
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        file.write(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    trace!("kernel:pid[{}] sys_read", current_task().unwrap().pid.0);
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        if !file.readable() {
            return -1;
        }
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        trace!("kernel: sys_read .. file.read");
        file.read(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_open(path: *const u8, flags: u32) -> isize {
    trace!("kernel:pid[{}] sys_open", current_task().unwrap().pid.0);
    let task = current_task().unwrap();
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(inode) = open_file(path.as_str(), OpenFlags::from_bits(flags).unwrap()) {
        let mut inner = task.inner_exclusive_access();
        let fd = inner.alloc_fd();
        inner.fd_table[fd] = Some(inode);
        fd as isize
    } else {
        -1
    }
}

pub fn sys_close(fd: usize) -> isize {
    trace!("kernel:pid[{}] sys_close", current_task().unwrap().pid.0);
    let task = current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    inner.fd_table[fd].take();
    0
}

/// YOUR JOB: Implement fstat.
pub fn sys_fstat(_fd: usize, _st: *mut Stat) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    println!(
        "kernel:pid[{}] sys_fstat NOT IMPLEMENTED",
        task.pid.0
    );
    if _fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[_fd] {
        println!("let some inside");
        let file = file.clone();
        println!("clone");
        let p = Arc::as_ptr(&file) as *const OSInode;
        println!("as ptr");
        // file.read(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
        unsafe {
            // (*p).inner.exclusive_access().inode;
            let node_id = (*p).inner.exclusive_access().inode.find_inode_id_by_inode();
            let count = (*p).inner.exclusive_access().inode.get_count();
            println!("node_id: {}, count: {}", node_id, count);
            
            let dev_addr = &((*_st).dev) as *const u64;
            let addr = translated_byte_buffer(token, dev_addr as *const u8, u64::BITS as usize);
            *(addr[0].as_ptr() as *mut u64) = 0u64;
            
            let ino_addr = &((*_st).ino) as *const u64;
            let addr = translated_byte_buffer(token, ino_addr as *const u8, u64::BITS as usize);
            *(addr[0].as_ptr() as *mut u64) = node_id as u64;
            
            let mode_addr = &((*_st).mode) as *const StatMode;
            let addr = translated_byte_buffer(token, mode_addr as *const u8, u32::BITS as usize);
            if (*p).inner.exclusive_access().inode.get_type() == 0 {
                *(addr[0].as_ptr() as *mut StatMode) = StatMode::FILE;
            } else {
                *(addr[0].as_ptr() as *mut StatMode) = StatMode::DIR;
            }
            
            let nlink_addr = &((*_st).nlink) as *const u32;
            let addr = translated_byte_buffer(token, nlink_addr as *const u8, u32::BITS as usize);
            *(addr[0].as_ptr() as *mut u32) = count as u32;


        }
        drop(inner);
        0
    } else {
        println!("let some return -1");
        -1
    }
}

/// YOUR JOB: Implement linkat.
pub fn sys_linkat(_old_name: *const u8, _new_name: *const u8) -> isize {
    trace!(
        "kernel:pid[{}] sys_linkat NOT IMPLEMENTED",
        current_task().unwrap().pid.0
    );
    let token = current_user_token();
    let old_path = translated_str(token, _old_name);
    let new_path = translated_str(token, _new_name);
    if old_path.eq(&new_path) {
        -1
    } else {
        link_file(old_path.as_str(), new_path.as_str());
        0
    }
}

/// YOUR JOB: Implement unlinkat.
pub fn sys_unlinkat(_name: *const u8) -> isize {
    trace!(
        "kernel:pid[{}] sys_unlinkat NOT IMPLEMENTED",
        current_task().unwrap().pid.0
    );
    let token = current_user_token();
    let path = translated_str(token, _name);
    unlink_file(path.as_str())
}
