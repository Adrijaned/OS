#include "io.h"
#include "rust_funcs.h"

void kmain(unsigned int a, unsigned int b) {
    load_gdt();
    rust_main(b, a);
}
