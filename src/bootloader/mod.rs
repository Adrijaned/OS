#[repr(C, packed)]
pub struct Multiboot1Structure {
    flags: u32,                             // 0
    mem_lower: u32,                         // 4
    mem_upper: u32,                         // 8
    boot_device: u32,                       // 12
    cmd_line: u32,                          // 16
    mods_count: u32,                        // 20
    mods_addr: u32,                         // 24
    syms: u128,                             // 28
    mmap_length: u32,                       // 44
    mmap_addr: u32,                         // 48
    drives_length: u32,                     // 52
    drives_addr: u32,                       // 56
    config_table: u32,                      // 60
    boot_loader_name: u32,                  // 64
    apm_table: u32,                         // 68
    vbe_table: VbeTable,                    // 72
    framebuffer_table: FramebufferTable,    // 88
}

#[repr(C, packed)]
struct VbeTable {
    vbe_control_info: u32,
    vbe_mode_info: u32,
    vbe_mode: u16,
    vbe_interface_seg: u16,
    vbe_interface_off: u16,
    vbe_interface_len: u16,
}

#[repr(C, packed)]
struct FramebufferTable {
    framebuffer_addr: u64,
    framebuffer_pitch: u32,
    framebuffer_width: u32,
    framebuffer_height: u32,
    framebuffer_bpp: u8,
    framebuffer_type: u8,
    color_info_1: u32,
    color_info_2: u16,
}

mod mmap;
pub use self::mmap::*;

impl Multiboot1Structure {
    pub fn mem_lower(&self) -> Result<u32, ()> {
        if self.flags & 0b1 != 1 {
            Err(())
        } else {
            Ok(self.mem_lower)
        }
    }
    pub fn mem_upper(&self) -> Result<u32, ()> {
        if self.flags & 0b1 != 1 {
            Err(())
        } else {
            Ok(self.mem_upper)
        }
    }
    pub fn boot_device(&self) -> Result<u32, ()> {
        if self.flags & 0b10 == 0 {
            Err(())
        } else {
            Ok(self.boot_device)
        }
    }
    pub fn cmd_line(&self) -> Result<u32, ()> {
        if self.flags & 0b100 == 0 {
            Err(())
        } else {
            Ok(self.cmd_line)
        }
    }
    pub fn mmap_length(&self) -> Result<u32, ()> {
        if self.flags & 0b1000000 == 0 {
            Err(())
        } else {
            Ok(self.mmap_length)
        }
    }
    pub fn mmap_addr(&self) -> Result<u32, ()> {
        if self.flags & 0b1000000 == 0 {
            Err(())
        } else {
            Ok(self.mmap_addr)
        }
    }
}