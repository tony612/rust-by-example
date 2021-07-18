// https://www.youtube.com/watch?v=b4mS5UPHh20&t=6s
// https://gist.github.com/jonhoo/935060885d0d832d463fda3c89e8259d

// Flavors:
//  - Synchronous channels: Channel where send() can block. Limited capacity.
//   - Mutex + Condvar + VecDeque
//   - Atomic VecDeque (atomic queue) + thread::park + thread::Thread::notify
//  - Asynchronous channels: Channel where send() cannot block. Unbounded.
//   - Mutex + Condvar + VecDeque
//   - Mutex + Condvar + LinkedList
//   - Atomic linked list, linked list of T
//   - Atomic block linked list, linked list of atomic VecDeque<T>
//  - Rendezvous channels: Synchronous with capacity = 0. Used for thread synchronization.
//  - Oneshot channels: Any capacity. In practice, only one call to send().

use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

// 1. Unbounded channel, channel to create tx and rx, send, recv
// 2. tx close
// 3. recv buffer

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Sender<T> {
    fn send(&self, v: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        let was_empty = inner.queue.is_empty();
        inner.queue.push_back(v);
        drop(inner);

        if was_empty {
            self.shared.have_item.notify_one();
        }
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);

        Sender {
            shared: self.shared.clone(),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let is_last = inner.senders == 0;
        drop(inner);
        if is_last {
            self.shared.have_item.notify_one();
        }
    }
}

struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>,
}

impl<T> Receiver<T> {
    fn recv(&mut self) -> Option<T> {
        if let Some(v) = self.buffer.pop_front() {
            return Some(v);
        }

        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(v) => {
                    if !inner.queue.is_empty() {
                        std::mem::swap(&mut inner.queue, &mut self.buffer);
                    }
                    return Some(v);
                }
                None if inner.senders == 0 => return None,
                None => {
                    inner = self.shared.have_item.wait(inner).unwrap();
                }
            }
        }
    }
}

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    have_item: Condvar,
}

fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::new(),
        senders: 1,
    };
    let shared = Shared {
        inner: Mutex::new(inner),
        have_item: Condvar::new(),
    };
    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
            buffer: VecDeque::new(),
        },
    )
}

#[cfg(test)]
mod test {
    use std::thread;
    use std::time;

    use super::*;

    #[test]
    fn works() {
        let (tx, mut rx) = channel();
        tx.send(1);
        tx.send(2);
        tx.send(3);
        assert_eq!(rx.recv(), Some(1));
        assert_eq!(rx.buffer.len(), 2);
        assert_eq!(rx.recv(), Some(2));
        assert_eq!(rx.buffer.len(), 1);
        assert_eq!(rx.recv(), Some(3));
        assert_eq!(rx.buffer.len(), 0);
        tx.send(4);
        assert_eq!(rx.recv(), Some(4));
        assert_eq!(rx.buffer.len(), 0);
    }

    #[test]
    fn recv_wait_and_notify() {
        let (tx, mut rx) = channel();
        thread::spawn(move || {
            thread::sleep(time::Duration::from_millis(5));
            tx.send(1);
        });
        // if not waiting, recv will likely fail because we tx send after 5ms
        assert_eq!(rx.recv(), Some(1));
    }

    #[test]
    fn tx_close() {
        let (tx, mut rx) = channel();
        tx.send(1);
        tx.send(2);
        drop(tx);
        assert_eq!(rx.recv(), Some(1));
        assert_eq!(rx.recv(), Some(2));
        assert_eq!(rx.recv(), None);
    }

    #[test]
    fn rx_close() {
        let (tx, rx) = channel();
        drop(rx);
        tx.send(42);
    }
}
