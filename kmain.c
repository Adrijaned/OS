#include "io.h"
#include "rust_funcs.h"

void kmain(void) {
    load_gdt();
    rust_main();
}
