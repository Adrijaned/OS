#[repr(C,packed)]
pub struct IdtEntry {
    offset_lower: u16,
    segment_selector: u16,
    zero: u8,
    type_attr: u8,
    offset_higher: u16,
}

impl IdtEntry {
    pub fn set_offset(&mut self, offset: u32) {
        self.offset_lower = (offset & 0xFFFF) as u16;
        self.offset_lower = ((offset >> 16) & 0xFFFF) as u16;
    }

    pub fn set_selector(&mut self, selector: u16) {
        self.segment_selector = selector;
    }
}