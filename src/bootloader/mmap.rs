#[repr(C,packed)]
pub struct MmapEntry {
    size: u32,
    base_addr_low: u32,
    base_addr_high: u32,
    length_low: u32,
    length_high: u32,
    entry_type: u32
}

impl super::super::io::Printable for MmapEntry {
    fn vga_print(&self) {
        print!("Size: ");
        println!(self.size);
        print!("Addr_low: ");
        println!(self.base_addr_low);
        print!("Addr_high: ");
        println!(self.base_addr_high);
        print!("Length_low: ");
        println!(self.length_low);
        print!("Length_high: ");
        println!(self.length_high);
        print!("Type: ");
        println!(self.entry_type);
    }
}

impl<'a> super::super::io::Printable for &'a MmapEntry {
    fn vga_print(&self) {
        print!("Size: ");
        println!(self.size);
        print!("Addr_low: ");
        println!(self.base_addr_low);
        print!("Addr_high: ");
        println!(self.base_addr_high);
        print!("Length_low: ");
        println!(self.length_low);
        print!("Length_high: ");
        println!(self.length_high);
        print!("Type: ");
        println!(self.entry_type);
    }
}