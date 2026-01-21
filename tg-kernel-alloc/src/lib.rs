//! 内存分配。

#![no_std]
// #![deny(warnings)]
#![deny(missing_docs)]

extern crate alloc;

use alloc::alloc::handle_alloc_error;
use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::NonNull,
};
use customizable_buddy::{BuddyAllocator, LinkedListBuddy, UsizeBuddy};

/// 初始化内存分配。
///
/// 参数 `base_address` 表示动态内存区域的起始位置。
///
/// # 注意
///
/// 此函数必须在使用任何堆分配之前调用，且只能调用一次。
#[inline]
pub fn init(base_address: usize) {
    // SAFETY: HEAP 是一个静态可变变量，此函数只在内核初始化时调用一次，
    // 此时没有其他代码会访问 HEAP。base_address 由调用者保证是有效的堆起始地址。
    unsafe {
        HEAP.init(
            core::mem::size_of::<usize>().trailing_zeros() as _,
            NonNull::new(base_address as *mut u8).unwrap(),
        )
    };
}

/// 将一个内存块托管到内存分配器。
///
/// # Safety
///
/// 调用者必须确保：
/// - `region` 内存块与已经转移到分配器的内存块都不重叠
/// - `region` 未被其他对象引用
/// - `region` 必须位于初始化时传入的起始位置之后
/// - 内存块的所有权将转移到分配器
#[inline]
pub unsafe fn transfer(region: &'static mut [u8]) {
    let ptr = NonNull::new(region.as_mut_ptr()).unwrap();
    // SAFETY: 由调用者保证内存块有效且不重叠
    HEAP.transfer(ptr, region.len());
}

/// 堆分配器。
///
/// 最大容量：6 + 21 + 3 = 30 -> 1 GiB。
/// 不考虑并发使用，因此没有加锁。
///
/// # Safety
///
/// 这是一个静态可变变量，仅在单处理器环境下使用。
/// 所有对 HEAP 的访问都通过 GlobalAlloc trait 进行，该 trait 本身是 unsafe 的。
static mut HEAP: BuddyAllocator<21, UsizeBuddy, LinkedListBuddy> = BuddyAllocator::new();

struct Global;

#[global_allocator]
static GLOBAL: Global = Global;

// SAFETY: GlobalAlloc 的实现必须是 unsafe 的。
// 此实现仅用于单处理器环境，不支持并发访问。
unsafe impl GlobalAlloc for Global {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: 在单处理器环境下，不会有并发的分配请求。
        // layout 的有效性由调用者（Rust 的 alloc 机制）保证。
        if let Ok((ptr, _)) = HEAP.allocate_layout::<u8>(layout) {
            ptr.as_ptr()
        } else {
            handle_alloc_error(layout)
        }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: 在单处理器环境下，不会有并发的释放请求。
        // ptr 和 layout 的有效性由调用者保证（必须是之前 alloc 返回的）。
        HEAP.deallocate_layout(NonNull::new(ptr).unwrap(), layout)
    }
}
