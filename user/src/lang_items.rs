use crate::println; 
use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(panic_info: &PanicInfo) -> ! {
    let msg = if let Some(message) = panic_info.message() {
        message.as_str().unwrap_or("(no message)")
    } else {
        "(no message)"
    };

    if let Some(location) = panic_info.location() {
        println!(
            "Panicked at {}:{}, {}",
            location.file(),
            location.line(),
            msg
        );
    } else {
        println!("Panicked: {}", msg);
    }
    
    loop {}
}