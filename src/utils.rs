use selinux_sys;

pub fn enabled() -> bool {
    match unsafe { selinux_sys::is_selinux_enabled() } {
        1 => true,
        ret => {
            debug_assert_eq!(ret, 0);
            false
        },
    }
}

pub fn mls_enabled() -> bool {
    match unsafe { selinux_sys::is_selinux_mls_enabled() } {
        1 => true,
        ret => {
            debug_assert_eq!(ret, 0);
            false
        },
    }
}
