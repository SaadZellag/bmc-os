use core::{
    mem::MaybeUninit,
    sync::atomic::{AtomicBool, AtomicI32, Ordering},
};

use alloc::{collections::VecDeque, vec::Vec};
use lazy_static::lazy_static;
use pc_keyboard::DecodedKey;
use ps2_mouse::MouseState;
use spin::Mutex;

use crate::println;

static mut EVENTS: MaybeUninit<SyncQueue> = MaybeUninit::uninit();

#[derive(Debug)]
struct Semaphore(AtomicI32);

#[derive(Debug)]
struct SyncQueue {
    elments: VecDeque<Event>,
    mutex: Semaphore,
    empty: Semaphore,
    full: Semaphore,
}

#[derive(Debug, Clone)]
pub enum Event {
    MouseInput(MouseState),
    KeyboardInput(DecodedKey),
}

impl Semaphore {
    fn wait(&self) {
        loop {
            let val = self.0.load(Ordering::Relaxed);

            if val <= 0 {
                continue;
            }

            let res = self
                .0
                .compare_exchange(val, val - 1, Ordering::Acquire, Ordering::Relaxed);

            if res.is_ok() {
                break;
            }
        }
    }

    fn signal(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

impl SyncQueue {
    fn add(&mut self, event: Event) {
        self.empty.wait();
        self.mutex.wait();
        self.elments.push_back(event);
        self.mutex.signal();
        self.full.signal();
    }

    fn poll(&mut self) -> Event {
        self.full.wait();
        self.mutex.wait();
        let res = self
            .elments
            .pop_front()
            .expect("Queue is empty while it should have at least an element");
        self.mutex.signal();
        self.empty.signal();
        res
    }

    fn new(size: i32) -> Self {
        Self {
            elments: VecDeque::with_capacity(size as usize),
            mutex: Semaphore(AtomicI32::new(1)),
            empty: Semaphore(AtomicI32::new(size)),
            full: Semaphore(AtomicI32::new(0)),
        }
    }
}

// unsafe impl Sync for
unsafe impl Sync for SyncQueue {}
unsafe impl Send for SyncQueue {}

// // Blocks until there is an event
pub fn next_event() -> Event {
    unsafe { EVENTS.assume_init_mut().poll() }
    // loop {
    //     if let Some(event) = try_next_event() {
    //         return event;
    //     }
    // }
}

// // Tries to get the next event if it exists
// pub fn try_next_event() -> Option<Event> {
//     if FLAG.compare_exchange(current, new, success, failure) {
//         unsafe { EVENTS.assume_init_read().pop_front() }
//     } else {
//         None
//     }
// }

// Blocks until it's added
pub fn add_event(event: Event) {
    unsafe { EVENTS.assume_init_mut().add(event) }
    // let mut events = EVENTS.lock();
    // events.push_back(event);
    // while let Some(e) = try_add_event(event) {
    //     event = e;
    // }
}

// pub fn try_add_event(event: Event) -> Option<Event> {
//     if let Some(mut events) = EVENTS.try_lock() {
//         events.push_back(event);
//         None
//     } else {
//         Some(event)
//     }
// }

pub fn init() {
    unsafe {
        EVENTS.write(SyncQueue::new(100));
    }
}
