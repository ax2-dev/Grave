use core::ffi::c_void;

use crate::consts;

#[repr(C)]
pub struct IMAGE_DOS_HEADER {
    pub e_magic: u16,
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    pub e_lfanew: i32,
}

#[repr(C)]
pub struct IMAGE_DATA_DIRECTORY {
    pub VirtualAddress: u32,
    pub Size: u32,
}

#[repr(C)]
pub struct IMAGE_FILE_HEADER {
    pub Machine: u16,
    pub NumberOfSections: u16,
    pub TimeDateStamp: u32,
    pub PointerToSymbolTable: u32,
    pub NumberOfSymbols: u32,
    pub SizeOfOptionalHeader: u16,
    pub Characteristics: u16,
}

#[repr(C)]
pub struct IMAGE_OPTIONAL_HEADER64 {
    pub Magic: u16,
    pub MajorLinkerVersion: u8,
    pub MinorLinkerVersion: u8,
    pub SizeOfCode: u32,
    pub SizeOfInitializedData: u32,
    pub SizeOfUninitializedData: u32,
    pub AddressOfEntryPoint: u32,
    pub BaseOfCode: u32,
    pub ImageBase: u64,
    pub SectionAlignment: u32,
    pub FileAlignment: u32,
    pub MajorOperatingSystemVersion: u16,
    pub MinorOperatingSystemVersion: u16,
    pub MajorImageVersion: u16,
    pub MinorImageVersion: u16,
    pub MajorSubsystemVersion: u16,
    pub MinorSubsystemVersion: u16,
    pub Win32VersionValue: u32,
    pub SizeOfImage: u32,
    pub SizeOfHeaders: u32,
    pub CheckSum: u32,
    pub Subsystem: u16,
    pub DllCharacteristics: u16,
    pub SizeOfStackReserve: u64,
    pub SizeOfStackCommit: u64,
    pub SizeOfHeapReserve: u64,
    pub SizeOfHeapCommit: u64,
    pub LoaderFlags: u32,
    pub NumberOfRvaAndSizes: u32,
    pub DataDirectory: [IMAGE_DATA_DIRECTORY; 16],
}

#[repr(C)]
pub struct IMAGE_NT_HEADERS64 {
    pub Signature: u32,
    pub FileHeader: IMAGE_FILE_HEADER,
    pub OptionalHeader: IMAGE_OPTIONAL_HEADER64,
}

#[repr(C)]
pub struct IMAGE_EXPORT_DIRECTORY {
    pub Characteristics: u32,
    pub TimeDateStamp: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub Name: u32,
    pub Base: u32,
    pub NumberOfFunctions: u32,
    pub NumberOfNames: u32,
    pub AddressOfFunctions: u32,
    pub AddressOfNames: u32,
    pub AddressOfNameOrdinals: u32,
}

const IMAGE_DIRECTORY_ENTRY_EXPORT: usize = 0;

const KEY: u32 = consts::COMPILETIME_SEED;

const BASE_OFFSET: u32 = 0xAC564B05;
const BASE_PRIME: u32 = 0x4B9210C9;

const DYNAMIC_OFFSET: u32 = BASE_OFFSET ^ KEY;
const DYNAMIC_PRIME: u32 = BASE_PRIME.wrapping_add(KEY);

#[inline(always)]
pub const fn const_hash(bytes: &[u8]) -> u32 {
    let mut hash = DYNAMIC_OFFSET;
    let mut i = 0;
    while i < bytes.len() {
        hash = hash ^ (bytes[i] as u32);
        hash = (hash << 5) | (hash >> 27); // Rotate Left
        hash = hash.wrapping_mul(DYNAMIC_PRIME);
        hash = hash.wrapping_add(bytes[i] as u32);
        i += 1;
    }
    hash
}

#[inline(always)]
pub fn runtime_hash(bytes: &[u8]) -> u32 {
    let mut hash = DYNAMIC_OFFSET;
    for &b in bytes {
        hash = hash ^ (b as u32);
        hash = (hash << 5) | (hash >> 27);
        hash = hash.wrapping_mul(DYNAMIC_PRIME);
        hash = hash.wrapping_add(b as u32);
    }
    hash
}

#[inline(always)]
pub fn get_proc_address_h(h_module: *mut c_void, target_hash: u32) -> Option<*const c_void> {
    unsafe {
        let base = h_module as *const u8;
        let dos_header = base as *const IMAGE_DOS_HEADER;

        if (*dos_header).e_magic != 0x5A4D {
            return None;
        }

        let nt_headers = base.offset((*dos_header).e_lfanew as isize) as *const IMAGE_NT_HEADERS64;

        if (*nt_headers).Signature != 0x00004550 {
            return None;
        }

        let export_dir_rva = (*nt_headers).OptionalHeader.DataDirectory
            [IMAGE_DIRECTORY_ENTRY_EXPORT as usize]
            .VirtualAddress;
        if export_dir_rva == 0 {
            return None;
        }

        let export_dir = base.offset(export_dir_rva as isize) as *const IMAGE_EXPORT_DIRECTORY;

        let names = base.offset((*export_dir).AddressOfNames as isize) as *const u32;
        let funcs = base.offset((*export_dir).AddressOfFunctions as isize) as *const u32;
        let ords = base.offset((*export_dir).AddressOfNameOrdinals as isize) as *const u16;

        for i in 0..(*export_dir).NumberOfNames {
            let name_rva = *names.offset(i as isize);
            let name_ptr = base.offset(name_rva as isize) as *const u8;

            let mut len = 0;
            while *name_ptr.add(len) != 0 {
                len += 1;
            }
            let name_slice = core::slice::from_raw_parts(name_ptr, len);

            if runtime_hash(name_slice) == target_hash {
                let ordinal = *ords.offset(i as isize) as usize;
                let func_rva = *funcs.offset(ordinal as isize);
                let func_ptr = base.offset(func_rva as isize);
                return Some(func_ptr as *const c_void);
            }
        }
        None
    }
}
