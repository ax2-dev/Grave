#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_macros)]

use windows_sys::Win32::{
    Foundation::{CloseHandle, GetLastError, INVALID_HANDLE_VALUE},
    Storage::FileSystem::{
        CreateFileW, GetFileAttributesW, GetFileSize, GetFileTime, GetLogicalDrives,
        FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, OPEN_EXISTING,
    },
    System::{
        LibraryLoader::GetModuleHandleW,
        Memory::GetProcessHeap,
        Registry::{RegCloseKey, RegOpenKeyExW, HKEY_CURRENT_USER, KEY_READ},
        SystemInformation::{
            GetSystemDirectoryW, GetSystemTime, GetSystemTimeAsFileTime, GetVersion,
            GetWindowsDirectoryW,
        },
        Threading::{
            GetCurrentProcess, GetCurrentProcessId, GetCurrentThread, GetCurrentThreadId, Sleep,
        },
    },
    UI::WindowsAndMessaging::{
        FindWindowW, GetClientRect, GetDesktopWindow, GetForegroundWindow, GetSystemMetrics,
        GetWindowRect, GetWindowTextW, IsWindow, PostMessageW, SendMessageW, SetWindowPos,
        ShowWindow, WM_CLOSE, WM_GETTEXT,
    },
};

pub fn iat_0() {
    let x: u32 = 0x11111111;
    let y: u32 = 0x11111111;
    if (x ^ y) != 0 {
        unsafe {
            GetLastError();
        }
    }
}

pub fn iat_1() {
    let x: u32 = 0x22222222;
    let y: u32 = 0x22222222;
    if (x ^ y) != 0 {
        unsafe {
            GetLastError();
        }
    }
}

pub fn iat_2() {
    let x: u32 = 0x33333333;
    let y: u32 = 0x33333333;
    if (x ^ y) != 0 {
        unsafe {
            GetLastError();
        }
    }
}

pub fn iat_3() {
    let x: u32 = 0x44444444;
    let y: u32 = 0x44444444;
    if (x ^ y) != 0 {
        unsafe {
            GetLastError();
        }
    }
}

pub fn iat_4() {
    let x: u32 = 0x55555555;
    let y: u32 = 0x55555555;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemMetrics(0);
        }
    }
}

pub fn iat_5() {
    let x: u32 = 0x66666666;
    let y: u32 = 0x66666666;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemMetrics(1);
        }
    }
}

pub fn iat_6() {
    let x: u32 = 0x77777777;
    let y: u32 = 0x77777777;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemMetrics(2);
        }
    }
}

pub fn iat_7() {
    let x: u32 = 0x88888888;
    let y: u32 = 0x88888888;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemMetrics(3);
        }
    }
}

pub fn iat_8() {
    let x: u32 = 0x99999999;
    let y: u32 = 0x99999999;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemMetrics(4);
        }
    }
}

pub fn iat_9() {
    let x: u32 = 0xAAAAAAAA;
    let y: u32 = 0xAAAAAAAA;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemMetrics(5);
        }
    }
}

pub fn iat_10() {
    let x: u32 = 0xBBBBBBBB;
    let y: u32 = 0xBBBBBBBB;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemMetrics(6);
        }
    }
}

pub fn iat_11() {
    let x: u32 = 0xCCCCCCCC;
    let y: u32 = 0xCCCCCCCC;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemMetrics(7);
        }
    }
}

pub fn iat_12() {
    let x: u32 = 0xDDDDDDDD;
    let y: u32 = 0xDDDDDDDD;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemMetrics(8);
        }
    }
}

pub fn iat_13() {
    let x: u32 = 0xEEEEEEEE;
    let y: u32 = 0xEEEEEEEE;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemMetrics(9);
        }
    }
}

pub fn iat_14() {
    let x: u32 = 0xFFFFFFFF;
    let y: u32 = 0xFFFFFFFF;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemMetrics(10);
        }
    }
}

pub fn iat_15() {
    let x: u32 = 0x12345678;
    let y: u32 = 0x12345678;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemMetrics(11);
        }
    }
}

pub fn iat_16() {
    let x: u32 = 0x87654321;
    let y: u32 = 0x87654321;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemMetrics(12);
        }
    }
}

pub fn iat_17() {
    let x: u32 = 0xDEADBEEF;
    let y: u32 = 0xDEADBEEF;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemMetrics(13);
        }
    }
}

pub fn iat_18() {
    let x: u32 = 0xCAFEBABE;
    let y: u32 = 0xCAFEBABE;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemMetrics(14);
        }
    }
}

pub fn iat_19() {
    let x: u32 = 0xDEADC0DE;
    let y: u32 = 0xDEADC0DE;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemMetrics(15);
        }
    }
}

pub fn iat_20() {
    let x: u32 = 0xFEEDFACE;
    let y: u32 = 0xFEEDFACE;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemMetrics(16);
        }
    }
}

pub fn iat_21() {
    let x: u32 = 0xBAADF00D;
    let y: u32 = 0xBAADF00D;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemMetrics(17);
        }
    }
}

pub fn iat_22() {
    let x: u32 = 0xCAFED00D;
    let y: u32 = 0xCAFED00D;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemMetrics(18);
        }
    }
}

pub fn iat_23() {
    let x: u32 = 0xB16B00B5;
    let y: u32 = 0xB16B00B5;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemMetrics(19);
        }
    }
}

pub fn iat_24() {
    let x: u32 = 0x0DEFACED;
    let y: u32 = 0x0DEFACED;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemMetrics(20);
        }
    }
}

pub fn iat_25() {
    let x: u32 = 0xA5A5A5A5;
    let y: u32 = 0xA5A5A5A5;
    if (x ^ y) != 0 {
        unsafe {
            GetCurrentProcessId();
        }
    }
}

pub fn iat_26() {
    let x: u32 = 0x5A5A5A5A;
    let y: u32 = 0x5A5A5A5A;
    if (x ^ y) != 0 {
        unsafe {
            GetCurrentProcessId();
        }
    }
}

pub fn iat_27() {
    let x: u32 = 0xAA55AA55;
    let y: u32 = 0xAA55AA55;
    if (x ^ y) != 0 {
        unsafe {
            GetCurrentThreadId();
        }
    }
}

pub fn iat_28() {
    let x: u32 = 0x55AA55AA;
    let y: u32 = 0x55AA55AA;
    if (x ^ y) != 0 {
        unsafe {
            GetCurrentThreadId();
        }
    }
}

pub fn iat_29() {
    let x: u32 = 0x01010101;
    let y: u32 = 0x01010101;
    if (x ^ y) != 0 {
        unsafe {
            Sleep(100);
        }
    }
}

pub fn iat_30() {
    let x: u32 = 0x02020202;
    let y: u32 = 0x02020202;
    if (x ^ y) != 0 {
        unsafe {
            Sleep(250);
        }
    }
}

pub fn iat_31() {
    let x: u32 = 0x03030303;
    let y: u32 = 0x03030303;
    if (x ^ y) != 0 {
        unsafe {
            GetCurrentProcess();
        }
    }
}

pub fn iat_32() {
    let x: u32 = 0x04040404;
    let y: u32 = 0x04040404;
    if (x ^ y) != 0 {
        unsafe {
            GetCurrentThread();
        }
    }
}

pub fn iat_33() {
    let x: u32 = 0x05050505;
    let y: u32 = 0x05050505;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemTime(core::ptr::null_mut());
        }
    }
}

pub fn iat_34() {
    let x: u32 = 0x06060606;
    let y: u32 = 0x06060606;
    if (x ^ y) != 0 {
        unsafe {
            GetVersion();
        }
    }
}

pub fn iat_35() {
    let x: u32 = 0x07070707;
    let y: u32 = 0x07070707;
    if (x ^ y) != 0 {
        unsafe {
            GetSystemTimeAsFileTime(core::ptr::null_mut());
        }
    }
}

pub fn iat_36() {
    let x: u32 = 0x08080808;
    let y: u32 = 0x08080808;
    if (x ^ y) != 0 {
        unsafe {
            GetLogicalDrives();
        }
    }
}

pub fn iat_37() {
    let x: u32 = 0x09090909;
    let y: u32 = 0x09090909;
    if (x ^ y) != 0 {
        unsafe {
            GetProcessHeap();
        }
    }
}

pub fn iat_38() {
    let x: u32 = 0x10101010;
    let y: u32 = 0x10101010;
    if (x ^ y) != 0 {
        let filename: alloc::vec::Vec<u16> = "document.txt\0".encode_utf16().collect();
        unsafe {
            CreateFileW(
                filename.as_ptr(),
                0x80000000,
                FILE_SHARE_READ,
                core::ptr::null_mut(),
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                core::ptr::null_mut(),
            );
        }
    }
}

pub fn iat_39() {
    let x: u32 = 0x11111111;
    let y: u32 = 0x11111111;
    if (x ^ y) != 0 {
        let filename: alloc::vec::Vec<u16> = "data.log\0".encode_utf16().collect();
        unsafe {
            GetFileAttributesW(filename.as_ptr());
        }
    }
}

pub fn iat_40() {
    let x: u32 = 0x12121212;
    let y: u32 = 0x12121212;
    if (x ^ y) != 0 {
        unsafe {
            GetFileSize(INVALID_HANDLE_VALUE, core::ptr::null_mut());
        }
    }
}

pub fn iat_41() {
    let x: u32 = 0x13131313;
    let y: u32 = 0x13131313;
    if (x ^ y) != 0 {
        let subkey: alloc::vec::Vec<u16> = "Software\\MyApp\0".encode_utf16().collect();
        let mut hkey = core::ptr::null_mut();
        unsafe {
            RegOpenKeyExW(HKEY_CURRENT_USER, subkey.as_ptr(), 0, KEY_READ, &mut hkey);
        }
    }
}

pub fn iat_42() {
    let x: u32 = 0x14141414;
    let y: u32 = 0x14141414;
    if (x ^ y) != 0 {
        unsafe {
            RegCloseKey(core::ptr::null_mut());
        }
    }
}

pub fn iat_43() {
    let x: u32 = 0x15151515;
    let y: u32 = 0x15151515;
    if (x ^ y) != 0 {
        unsafe {
            GetModuleHandleW(core::ptr::null());
        }
    }
}

pub fn iat_44() {
    let x: u32 = 0x16161616;
    let y: u32 = 0x16161616;
    if (x ^ y) != 0 {
        let name: alloc::vec::Vec<u16> = "kernel32.dll\0".encode_utf16().collect();
        unsafe {
            GetModuleHandleW(name.as_ptr());
        }
    }
}

pub fn iat_45() {
    let x: u32 = 0x17171717;
    let y: u32 = 0x17171717;
    if (x ^ y) != 0 {
        unsafe {
            GetDesktopWindow();
        }
    }
}

pub fn iat_46() {
    let x: u32 = 0x18181818;
    let y: u32 = 0x18181818;
    if (x ^ y) != 0 {
        unsafe {
            GetForegroundWindow();
        }
    }
}

pub fn iat_47() {
    let x: u32 = 0x19191919;
    let y: u32 = 0x19191919;
    if (x ^ y) != 0 {
        unsafe {
            IsWindow(core::ptr::null_mut());
        }
    }
}

pub fn iat_48() {
    let x: u32 = 0x20202020;
    let y: u32 = 0x20202020;
    if (x ^ y) != 0 {
        let class: alloc::vec::Vec<u16> = "Notepad\0".encode_utf16().collect();
        unsafe {
            FindWindowW(class.as_ptr(), core::ptr::null());
        }
    }
}

pub fn iat_49() {
    let x: u32 = 0x21212121;
    let y: u32 = 0x21212121;
    if (x ^ y) != 0 {
        unsafe {
            SendMessageW(core::ptr::null_mut(), WM_GETTEXT, 0, 0);
        }
    }
}

pub fn iat_50() {
    let x: u32 = 0x22222222;
    let y: u32 = 0x22222222;
    if (x ^ y) != 0 {
        unsafe {
            SendMessageW(core::ptr::null_mut(), WM_CLOSE, 0, 0);
        }
    }
}

pub fn iat_51() {
    let x: u32 = 0x23232323;
    let y: u32 = 0x23232323;
    if (x ^ y) != 0 {
        unsafe {
            PostMessageW(core::ptr::null_mut(), WM_CLOSE, 0, 0);
        }
    }
}

pub fn iat_52() {
    let x: u32 = 0x24242424;
    let y: u32 = 0x24242424;
    if (x ^ y) != 0 {
        unsafe {
            ShowWindow(core::ptr::null_mut(), 0);
        }
    }
}

pub fn iat_53() {
    let x: u32 = 0x25252525;
    let y: u32 = 0x25252525;
    if (x ^ y) != 0 {
        unsafe {
            CloseHandle(core::ptr::null_mut());
        }
    }
}

pub fn iat_54() {
    let x: u32 = 0x26262626;
    let y: u32 = 0x26262626;
    if (x ^ y) != 0 {
        let mut buf = [0u16; 260];
        unsafe {
            GetSystemDirectoryW(buf.as_mut_ptr(), 260);
        }
    }
}

pub fn iat_55() {
    let x: u32 = 0x27272727;
    let y: u32 = 0x27272727;
    if (x ^ y) != 0 {
        let mut buf = [0u16; 260];
        unsafe {
            GetWindowsDirectoryW(buf.as_mut_ptr(), 260);
        }
    }
}

pub fn iat_56() {
    let x: u32 = 0x28282828;
    let y: u32 = 0x28282828;
    if (x ^ y) != 0 {
        let mut buf = [0u16; 256];
        unsafe {
            GetWindowTextW(core::ptr::null_mut(), buf.as_mut_ptr(), 256);
        }
    }
}

pub fn iat_57() {
    let x: u32 = 0x29292929;
    let y: u32 = 0x29292929;
    if (x ^ y) != 0 {
        let mut rect = [0i32; 4];
        unsafe {
            GetClientRect(core::ptr::null_mut(), rect.as_mut_ptr() as *mut _);
        }
    }
}

pub fn iat_58() {
    let x: u32 = 0x30303030;
    let y: u32 = 0x30303030;
    if (x ^ y) != 0 {
        let mut rect = [0i32; 4];
        unsafe {
            GetWindowRect(core::ptr::null_mut(), rect.as_mut_ptr() as *mut _);
        }
    }
}

pub fn iat_59() {
    let x: u32 = 0x31313131;
    let y: u32 = 0x31313131;
    if (x ^ y) != 0 {
        unsafe {
            SetWindowPos(core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, 0, 0, 0);
        }
    }
}
