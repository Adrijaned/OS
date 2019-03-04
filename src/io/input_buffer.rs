/// last element is top of current contents
static mut BUFFER: [char; 128] = [0 as char; 128];
pub fn register_keypress(c: char) {
    ::io::putchar(c);

}