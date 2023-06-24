use core::mem::MaybeUninit;

#[derive(Debug)]
pub struct Queue<T, const N: usize> {
    elems: [MaybeUninit<T>; N],
    front: usize,
    back: usize,
    len: usize,
}

impl<T, const N: usize> Queue<T, N> {
    pub const fn new() -> Self {
        Self {
            elems: unsafe { MaybeUninit::uninit().assume_init() },
            front: 0,
            back: 0,
            len: 0,
        }
    }

    pub fn push_back(&mut self, item: T) -> Option<T> {
        if self.len == N {
            return Some(item);
        }
        self.len += 1;
        self.elems[self.back] = MaybeUninit::new(item);
        self.back = (self.back + 1) % N;

        None
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        let res = unsafe { self.elems[self.front].assume_init_read() };
        self.front = (self.front + N - 1) % N;
        Some(res)
    }
}
