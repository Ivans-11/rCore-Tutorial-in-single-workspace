use super::{MonoForeignPortal, PortalCache, PORTAL_TEXT};

/// 包含多个插槽的异界传送门。
#[repr(C)]
pub struct MultislotPortal {
    slot_count: usize,
    text_size: usize,
}

macro_rules! sizeof {
    ($ty:ty) => {
        core::mem::size_of::<$ty>()
    };
}

impl MultislotPortal {
    /// 计算包括 `slots` 个插槽的传送门总长度。
    #[inline]
    pub fn calculate_size(slots: usize) -> usize {
        sizeof!(Self) + PORTAL_TEXT.aligned_size() + slots * sizeof!(PortalCache)
    }

    /// 初始化公共空间上的传送门。
    ///
    /// # Safety
    ///
    /// 调用者必须确保：
    /// - `transit` 是一个正确映射到公共地址空间上的地址
    /// - `transit` 指向的内存区域大小至少为 `calculate_size(slots)` 字节
    /// - `transit` 地址满足 `usize` 对齐要求
    /// - 该内存区域具有读、写、执行权限
    pub unsafe fn init_transit(transit: usize, slots: usize) -> &'static mut Self {
        // 判断 transit 满足对齐要求
        debug_assert!(transit.trailing_zeros() > sizeof!(usize).trailing_zeros());
        // SAFETY: 由调用者保证 transit 指向足够大小的有效内存
        PORTAL_TEXT.copy_to(transit + sizeof!(Self));
        // SAFETY: 由调用者保证 transit 对齐且指向有效内存，
        // 返回 'static 生命周期是因为传送门在整个内核运行期间都有效
        let ans = &mut *(transit as *mut Self);
        ans.slot_count = slots;
        ans.text_size = PORTAL_TEXT.aligned_size();
        ans
    }
}

impl MonoForeignPortal for MultislotPortal {
    #[inline]
    fn total_size(&self) -> usize {
        self.cache_offset(self.slot_count)
    }

    #[inline]
    fn transit_address(&self) -> usize {
        self as *const _ as usize
    }

    #[inline]
    fn text_offset(&self) -> usize {
        sizeof!(Self)
    }

    #[inline]
    fn cache_offset(&self, key: usize) -> usize {
        sizeof!(Self) + self.text_size + key * sizeof!(PortalCache)
    }
}
