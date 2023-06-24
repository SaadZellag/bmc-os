use core::{
    mem::MaybeUninit,
    sync::atomic::{AtomicBool, AtomicI32, Ordering},
};

use alloc::{collections::VecDeque, vec::Vec};
use lazy_static::lazy_static;
use pc_keyboard::DecodedKey;
use ps2_mouse::MouseState;
use spin::Mutex;

use crate::{println, queue::Queue};

static mut EVENTS: SyncQueue<100> = SyncQueue::new();

#[derive(Debug)]
struct Semaphore(AtomicI32);

#[derive(Debug)]
struct SyncQueue<const N: usize> {
    elments: Queue<Event, N>,
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
                .compare_exchange(val, val - 1, Ordering::Relaxed, Ordering::Relaxed);

            if res.is_ok() {
                break;
            }
        }
    }

    fn signal(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

impl<const N: usize> SyncQueue<N> {
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

    const fn new() -> Self {
        Self {
            elments: Queue::new(),
            mutex: Semaphore(AtomicI32::new(1)),
            empty: Semaphore(AtomicI32::new(N as i32)),
            full: Semaphore(AtomicI32::new(0)),
        }
    }
}

// Blocks until there is an event
pub fn next_event() -> Event {
    unsafe { EVENTS.poll() }
}

// Blocks until event is added
pub fn add_event(event: Event) {
    unsafe { EVENTS.add(event) }
}
