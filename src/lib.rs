#![feature(lang_items)]
#![feature(const_fn)]
#![feature(ptr_internals)]
#![feature(const_unique_new)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn rust_main() {
    // NOTE: we have a very small stack and no guard page

    vga_buffer::clear_screen();
    println!("Hello World{}", "!");
    println!("{}", { println!("inner"); "outer" });

    loop {}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! { loop{} }
