use crate::{display::get_current_text_color, gdt, print, println, set_text_color};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use ps2_mouse::{Mouse, MouseState};
use spin::{self, Lazy, Mutex};
use vga::colors::Color16;
use x86_64::{
    instructions::port::PortReadOnly,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
    Mouse = PIC_2_OFFSET + 4,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt.page_fault.set_handler_fn(page_fault_handler);

        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt[InterruptIndex::Mouse.as_usize()].set_handler_fn(mouse_interrupt_handler);
        idt
    };
    static ref MOUSE: Mutex<Mouse> = Mutex::new(Mouse::new());
}

pub fn init_idt() {
    IDT.load();
    init_mouse();
}

fn init_mouse() {
    MOUSE.lock().init().unwrap();
    MOUSE.lock().set_on_complete(on_complete);
}

// This will be fired when a packet is finished being processed.
fn on_complete(mouse_state: MouseState) {
    println!("{:?}", mouse_state);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    let color = get_current_text_color();
    set_text_color!(Color16::Yellow, Color16::Black);
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    set_text_color!(color);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    set_text_color!(Color16::Yellow, Color16::Black);
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
        );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

use x86_64::structures::idt::PageFaultErrorCode;

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    set_text_color!(Color16::Yellow, Color16::Black);

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    loop {}
}

// An example interrupt based on https://os.phil-opp.com/hardware-interrupts/. The ps2 mouse is configured to fire
// interrupts at PIC offset 12.
extern "x86-interrupt" fn mouse_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut port = PortReadOnly::new(0x60);
    let packet = unsafe { port.read() };
    MOUSE.lock().process_packet(packet);

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Mouse as u8);
    }
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}
