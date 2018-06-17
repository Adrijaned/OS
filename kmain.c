#include "io.h"
#include "rust_funcs.h"

typedef enum {
    BLACK,
    BLUE,
    GREEN,
    CYAN,
    RED,
    MAGENTA,
    BROWN,
    LIGHT_GREY,
    DARK_GREY,
    LIGHT_BLUE,
    LIGHT_GREEN,
    LIGHT_CYAN,
    LIGHT_RED,
    LIGHT_MAGENTA,
    LIGHT_BROWN,
    WHITE
} TextCellColorEnum;

char* TEXT_MEMORY_START = (char*)0x000B8000;

// [ 15 . . . . . . 8 | 7 . . 4 | 3 . . 0 ]
// [char to be written|fg colour|bg colour]
void fb_write_cell(unsigned short index, char character, TextCellColorEnum foreground_color, TextCellColorEnum background_color) {
    if (index >= 2000) return;
    TEXT_MEMORY_START[2*index] = character;
    TEXT_MEMORY_START[2*index + 1] = ((background_color << 4) | foreground_color);
}

/* The I/O ports */
#define FB_COMMAND_PORT         0x3D4
#define FB_DATA_PORT            0x3D5

/* The I/O port commands */
#define FB_HIGH_BYTE_COMMAND    14
#define FB_LOW_BYTE_COMMAND     15

/** fb_move_cursor:
 *  Moves the cursor of the framebuffer to the given position
 *
 *  @param pos The new position of the cursor
 */
void fb_move_cursor(unsigned short pos) {
    outb(FB_COMMAND_PORT, FB_HIGH_BYTE_COMMAND);
    outb(FB_DATA_PORT,    ((pos >> 8) & 0x00FF));
    outb(FB_COMMAND_PORT, FB_LOW_BYTE_COMMAND);
    outb(FB_DATA_PORT,    pos & 0x00FF);
}

void kmain(void) {
    fb_write_cell(10, 'f', LIGHT_BLUE, LIGHT_BROWN);
    rust_main();
}
