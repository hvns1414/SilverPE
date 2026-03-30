//GitHub: https://github.com/hvns1414/SilverPE
use std::ffi::CStr;

#[repr(C)]
#[derive(Default, Copy, Clone)]
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
#[derive(Default, Copy, Clone)]
pub struct IMAGE_FILE_HEADER {
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
}

pub const IMAGE_FILE_32BIT_MACHINE: u16 = 0x0100;

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct IMAGE_DATA_DIRECTORY {
    pub virtual_address: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct IMAGE_OPTIONAL_HEADER32 {
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub base_of_data: u32,
    pub image_base: u32,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version_value: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub check_sum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u32,
    pub size_of_stack_commit: u32,
    pub size_of_heap_reserve: u32,
    pub size_of_heap_commit: u32,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
    pub export_table: IMAGE_DATA_DIRECTORY,
    pub import_table: IMAGE_DATA_DIRECTORY,
    pub resource_table: IMAGE_DATA_DIRECTORY,
    pub exception_table: IMAGE_DATA_DIRECTORY,
    pub certificate_table: IMAGE_DATA_DIRECTORY,
    pub base_relocation_table: IMAGE_DATA_DIRECTORY,
    pub debug: IMAGE_DATA_DIRECTORY,
    pub architecture: IMAGE_DATA_DIRECTORY,
    pub global_ptr: IMAGE_DATA_DIRECTORY,
    pub tls_table: IMAGE_DATA_DIRECTORY,
    pub load_config_table: IMAGE_DATA_DIRECTORY,
    pub bound_import: IMAGE_DATA_DIRECTORY,
    pub iat: IMAGE_DATA_DIRECTORY,
    pub delay_import_descriptor: IMAGE_DATA_DIRECTORY,
    pub clr_runtime_header: IMAGE_DATA_DIRECTORY,
    pub reserved: IMAGE_DATA_DIRECTORY,
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct IMAGE_OPTIONAL_HEADER64 {
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub image_base: u64,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version_value: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub check_sum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u64,
    pub size_of_stack_commit: u64,
    pub size_of_heap_reserve: u64,
    pub size_of_heap_commit: u64,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
    pub export_table: IMAGE_DATA_DIRECTORY,
    pub import_table: IMAGE_DATA_DIRECTORY,
    pub resource_table: IMAGE_DATA_DIRECTORY,
    pub exception_table: IMAGE_DATA_DIRECTORY,
    pub certificate_table: IMAGE_DATA_DIRECTORY,
    pub base_relocation_table: IMAGE_DATA_DIRECTORY,
    pub debug: IMAGE_DATA_DIRECTORY,
    pub architecture: IMAGE_DATA_DIRECTORY,
    pub global_ptr: IMAGE_DATA_DIRECTORY,
    pub tls_table: IMAGE_DATA_DIRECTORY,
    pub load_config_table: IMAGE_DATA_DIRECTORY,
    pub bound_import: IMAGE_DATA_DIRECTORY,
    pub iat: IMAGE_DATA_DIRECTORY,
    pub delay_import_descriptor: IMAGE_DATA_DIRECTORY,
    pub clr_runtime_header: IMAGE_DATA_DIRECTORY,
    pub reserved: IMAGE_DATA_DIRECTORY,
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct IMAGE_SECTION_HEADER {
    pub name: [u8; 8],
    pub physical_address_or_virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_linenumbers: u32,
    pub number_of_relocations: u16,
    pub number_of_linenumbers: u16,
    pub characteristics: u32,
}

impl IMAGE_SECTION_HEADER {
    pub fn name_str(&self) -> String {
        let mut name = String::new();
        for &b in &self.name {
            if b == 0 { break; }
            name.push(b as char);
        }
        name
    }
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct IMAGE_BASE_RELOCATION {
    pub virtual_address: u32,
    pub size_of_block: u32,
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct IMAGE_IMPORT_DESCRIPTOR {
    pub original_first_thunk: u32,
    pub time_date_stamp: u32,
    pub forwarder_chain: u32,
    pub name: u32,
    pub first_thunk: u32,
}

pub unsafe fn read_struct<T>(base: *const u8, offset: usize) -> T {
    std::ptr::read_unaligned(base.add(offset) as *const T)
}

pub unsafe fn read_ansi_string(ptr: *const u8) -> String {
    let c_str = CStr::from_ptr(ptr as *const i8);
    c_str.to_string_lossy().into_owned()
}
