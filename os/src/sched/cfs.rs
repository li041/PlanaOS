use core::sync::atomic::AtomicIsize;

use super::Scheduler;



pub struct CFSTask<T> {
    inner: T, 
    vruntime: AtomicIsize,
    nice: AtomicIsize,
}

impl <T> CFSTask<T> {
    pub const fn new(inner: T) -> Self {
        CFSTask {
            inner,
            vruntime: AtomicIsize::new(0),
            nice: AtomicIsize::new(0),
        }
    }

}