#![no_std]
#![no_main]

use core::arch::global_asm;

global_asm!(
    r#"
    .section .text
    .global _start
    .type _start, function
    _start:
        ldr r0, =_stack_start
        msr msp, r0
        b main
    .size _start, . - _start
    "#
);

global_asm!(
    r#"
    .section .text
    .global read_reg_asm
    .type read_reg_asm, function
    read_reg_asm:
        ldr r0, [r0]
        bx lr
    .size read_reg_asm, . - read_reg_asm
    "#
);
extern "C" {
    fn _start();
    fn _stack_start();
}
#[doc(hidden)]
#[repr(C)]
pub union Vector {
    handler: unsafe extern "C" fn(),
    reserved: usize,
}

#[link_section = ".isr_vector"]
pub static VEC: [Vector; 2] = [
    Vector {
        handler: _stack_start,
    },
    Vector { handler: _start },
];

const PERIPH_BASE: usize = 0x40000000;
const AHB1PERIPH_BASE: usize = PERIPH_BASE + 0x00020000;
const RCC_BASE: usize = AHB1PERIPH_BASE + 0x00003800;
const RCC_AHB1ENR: usize = RCC_BASE + 0x30;

const GPIOA_BASE: usize = AHB1PERIPH_BASE + 0x0000;
const GPIOC_BASE: usize = AHB1PERIPH_BASE + 0x0800;

const GPIOA_MODER: usize = GPIOA_BASE + 0x00;
const GPIOA_PUPDR: usize = GPIOA_BASE + 0x0C;
const GPIOA_IDR: usize = GPIOA_BASE + 0x10;

const GPIOC_MODER: usize = GPIOC_BASE + 0x00;
const GPIOC_BSRR: usize = GPIOC_BASE + 0x18;

fn read_reg(addr: usize) -> usize {
    extern "C" {
        fn read_reg_asm(addr: usize) -> usize;
    }
    unsafe { read_reg_asm(addr) }
}
fn write_reg(addr: usize, value: usize) {
    unsafe { core::arch::asm!("str r1, [r0]", in("r0") addr, in("r1") value) }
}

fn init_pa0_as_input() {
    // Enable GPIOA clock
    write_reg(RCC_AHB1ENR, read_reg(RCC_AHB1ENR) | (1 << 0));
    // Set PA0 mode to input
    write_reg(GPIOA_PUPDR, 1);
    write_reg(GPIOA_MODER, 0);
}
fn read_pa0() -> bool {
    read_reg(GPIOA_IDR) & 0x01 == 0x01
}
fn init_pc13_as_output() {
    // Enable GPIOC clock
    write_reg(RCC_AHB1ENR, read_reg(RCC_AHB1ENR) | (1 << 2));
    // Set PC13 mode to output
    write_reg(GPIOC_MODER, 0x01 << (13 * 2));
}
fn write_pc13(value: bool) {
    if value {
        write_reg(GPIOC_BSRR, 1 << 13);
    } else {
        write_reg(GPIOC_BSRR, 1 << (16 + 13));
    }
}
#[no_mangle]
fn main() {
    init_pa0_as_input();
    init_pc13_as_output();
    loop {
        write_pc13(read_pa0());
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
