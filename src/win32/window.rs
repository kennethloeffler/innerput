use std::{ffi::CString, process, ptr, time::Duration};

use winapi::{
    shared::{
        minwindef::{BOOL, DWORD, FALSE, LPARAM, TRUE},
        windef::HWND,
    },
    um::{
        processthreadsapi::GetCurrentThreadId,
        winuser::{
            AttachThreadInput, EnumWindows, FindWindowA, GetForegroundWindow, GetWindow,
            GetWindowThreadProcessId, IsHungAppWindow, IsIconic, IsWindowVisible,
            SetForegroundWindow, ShowWindow, GW_OWNER, SW_RESTORE,
        },
    },
};

const SLEEP_DURATION: u64 = 10;

#[derive(Eq, PartialEq)]
struct Window {
    hwnd: HWND,
}

impl Window {
    pub unsafe fn from_raw_handle(hwnd: HWND) -> Self {
        Self { hwnd }
    }

    pub fn process_id(&self) -> DWORD {
        let mut process_id: DWORD = 0;
        unsafe { GetWindowThreadProcessId(self.hwnd, &mut process_id as *mut _) };
        process_id
    }

    pub fn thread_id(&self) -> DWORD {
        unsafe { GetWindowThreadProcessId(self.hwnd, ptr::null_mut()) }
    }

    pub fn is_visible(&self) -> bool {
        let result = unsafe { IsWindowVisible(self.hwnd) };
        result != 0
    }

    pub fn try_set_foreground(&self, current_foreground_window: &Window) -> Option<()> {
        // Autohotkey source:
        // > Probably best not to trust its return value.  It's been shown to be
        // > unreliable at times.  Example: I've confirmed that
        // > SetForegroundWindow() sometimes (perhaps about 10% of the time)
        // > indicates failure even though it succeeds.
        unsafe { SetForegroundWindow(self.hwnd) };

        std::thread::sleep(Duration::from_millis(SLEEP_DURATION));

        let new_foreground_window = get_foreground_window();

        if new_foreground_window == *self
            || new_foreground_window != *current_foreground_window
                && self.hwnd == unsafe { GetWindow(new_foreground_window.hwnd, GW_OWNER) }
        {
            Some(())
        } else {
            None
        }
    }

    pub fn is_foreground(&self) -> bool {
        let foreground = unsafe { GetForegroundWindow() };
        foreground == self.hwnd
    }

    pub fn is_minimized(&self) -> bool {
        let result = unsafe { IsIconic(self.hwnd) };
        result != 0
    }

    pub fn is_hung(&self) -> bool {
        let result = unsafe { IsHungAppWindow(self.hwnd) };
        result != 0
    }

    pub fn restore_if_minimized(&self) {
        if self.is_minimized() {
            unsafe { ShowWindow(self.hwnd, SW_RESTORE) };
        }
    }
}

fn enum_windows<F>(mut func: F)
where
    F: FnMut(Window) -> bool,
{
    unsafe extern "system" fn callback<F>(hwnd: HWND, func_ptr: LPARAM) -> BOOL
    where
        F: FnMut(Window) -> bool,
    {
        let func: &mut &mut F = &mut *(func_ptr as *mut _);
        func(Window::from_raw_handle(hwnd)) as BOOL
    }

    let func_ptr = &mut &mut func as *mut _;
    let lparam = func_ptr as LPARAM;

    unsafe { EnumWindows(Some(callback::<F>), lparam) };
}

fn get_foreground_window() -> Window {
    let mut foreground_hwnd = unsafe { GetForegroundWindow() };

    // The taskbar is focused if the foreground window is null.
    if foreground_hwnd.is_null() {
        foreground_hwnd = unsafe {
            let window_name = CString::new("Shell_TrayWnd").unwrap();
            FindWindowA(window_name.as_ptr(), ptr::null_mut())
        }
    }

    unsafe { Window::from_raw_handle(foreground_hwnd) }
}

fn get_top_level_window(process: &process::Child) -> Option<Window> {
    let mut result_hwnd = ptr::null_mut();

    enum_windows(|window| {
        if window.process_id() == process.id() && window.is_visible() {
            result_hwnd = window.hwnd;
            false
        } else {
            true
        }
    });

    if !result_hwnd.is_null() {
        Some(unsafe { Window::from_raw_handle(result_hwnd) })
    } else {
        None
    }
}

pub fn activate_top_level_window(process: &process::Child) -> Option<()> {
    if let Some(window) = get_top_level_window(process) {
        // Calling SetForegroundWindow on a hung window while AttachThreadInput
        // is in effect can cause us to hang too!
        if window.is_hung() {
            return None;
        }

        // The target window is already in the foreground, so nothing more needs
        // to be done.
        if window.is_foreground() {
            return Some(());
        }

        let foreground_window = get_foreground_window();
        let foreground_thread = foreground_window.thread_id();
        let target_thread = window.thread_id();
        let current_thread = unsafe { GetCurrentThreadId() };

        window.restore_if_minimized();

        let did_attach_current_to_foreground = !foreground_window.is_hung()
            && unsafe { AttachThreadInput(current_thread, foreground_thread, TRUE) } != FALSE;

        let did_attach_foreground_to_target =
            unsafe { AttachThreadInput(foreground_thread, target_thread, TRUE) } != FALSE;

        // The AutoHotkey source mentions that this "never seems to take more
        // than two tries" and that "the number of tries needed might vary
        // depending on how fast the CPU is." I don't know...
        for _ in 0..5 {
            if let Some(()) = window.try_set_foreground(&foreground_window) {
                break;
            } else {
                continue;
            };
        }

        if did_attach_current_to_foreground {
            unsafe { AttachThreadInput(current_thread, foreground_thread, FALSE) };
        };

        if did_attach_foreground_to_target {
            unsafe { AttachThreadInput(foreground_thread, target_thread, FALSE) };
        };

        Some(())
    } else {
        None
    }
}
