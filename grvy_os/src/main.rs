#![no_std] // don't link to the Rust std library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
mod vga_buffer;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;

    println!("Hello World{}", "!");
    write!(
        vga_buffer::WRITER.lock(),
        "The numbers are {} and {}",
        42,
        1.337
    )
    .unwrap();
    loop {}
}

// this function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
