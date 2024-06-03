use std::cmp::Ord;
use std::fmt::Debug;
use std::clone::Clone;

#[derive(Debug)]
pub struct MinHeap<T: Ord + Clone + Debug> {
    data: Vec<T>,
}

impl<T: Ord + Clone + Debug> MinHeap<T> {
    pub fn new() -> Self {
        MinHeap { data: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.heapify_up(self.data.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            None
        } else {
            let min = self.data.swap_remove(0);
            if !self.data.is_empty() {
                self.heapify_down(0);
            }
            Some(min)
        }
    }

    pub fn empty(&self) -> bool{
        self.data.is_empty()
    }

    pub fn top(&self) -> Option<T> {
        self.data.get(0).cloned()
    }

    fn heapify_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.data[parent] <= self.data[index] {
                break;
            }
            self.data.swap(parent, index);
            index = parent;
        }
    }

    fn heapify_down(&mut self, mut index: usize) {
        let len = self.data.len();
        loop {
            let child = 2 * index + 1;
            if child >= len {
                break;
            }
            let mut min_child = child;
            if child + 1 < len && self.data[child] > self.data[child + 1] {
                min_child = child + 1;
            }
            if self.data[min_child] >= self.data[index] {
                break;
            }
            self.data.swap(index, min_child);
            index = min_child;
        }
    }
}
