use crate::{
    config::PAGE_SIZE_BITS,
    mm::{MapPermission, VirtAddr, VirtPageNum},
    task::current_task,
    utils::{ceil_to_page_size, floor_to_page_size},
};

pub fn sys_brk(brk: usize) -> isize {
    log::info!("sys_brk: brk: {:#x}", brk);
    let current_task = current_task();
    // sbrk(0)是获取当前program brk(堆顶)
    if brk == 0 {
        return current_task.inner.lock().memory_set.brk as isize;
    }
    let current_mm = &mut current_task.inner.lock().memory_set;
    let current_brk = current_mm.brk;
    let heap_bottom = current_mm.heap_bottom;
    // (start_vpn, end_vpn)是需要增/删的区间
    let start_vpn = VirtPageNum::from(ceil_to_page_size(current_brk) >> PAGE_SIZE_BITS);
    let new_end_vpn = VirtPageNum::from(ceil_to_page_size(brk) >> PAGE_SIZE_BITS);
    if brk < heap_bottom {
        // brk小于堆底, 不合法
        log::error!("[sys_brk] brk {:#x} < heap_bottom {:#x}", brk, heap_bottom);
        return -1;
    } else if brk > ceil_to_page_size(current_brk) {
        // 需要分配页
        if current_brk == heap_bottom {
            // 初始分配堆空间
            log::info!(
                "[sys_brk] init heap space: {:#x} - {:#x}",
                heap_bottom,
                new_end_vpn.0 << PAGE_SIZE_BITS
            );
            current_mm.insert_framed_area(
                VirtAddr::from(heap_bottom),
                VirtAddr::from(new_end_vpn.0 << PAGE_SIZE_BITS),
                MapPermission::R | MapPermission::W | MapPermission::U,
                0,
            );
        } else {
            current_mm.remap_area_with_start_vpn(start_vpn, new_end_vpn);
        }
    } else if brk < floor_to_page_size(current_brk) {
        // 需要释放页, 若start_vpn == new_end_vpn, 会将空间删除
        current_mm.remap_area_with_start_vpn(start_vpn, new_end_vpn);
    } else {
        // brk在同一页, 不用alloc/dealloc页
        // 页内偏移
    }
    current_mm.brk = brk;
    0
}
