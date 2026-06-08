#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::yield_;

static mut COUNTER: usize = 0;

#[no_mangle]
fn main() -> i32 {
    for round in 1..=5 {
        unsafe {
            COUNTER += 1;
        }
        let (counter_addr, counter_val) = unsafe {
            (&COUNTER as *const _ as usize, COUNTER)
        };
        println!(
            "mem_isolation [round {}] counter @ {:#x} = {}",
            round, counter_addr, counter_val,
        );
        yield_();
    }
    println!("Test mem_isolation OK!");
    0
}
