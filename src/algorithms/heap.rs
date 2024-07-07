//! # 堆
//!
//!

use std::slice::Iter;

pub struct Heap<T> {
    values: Vec<T>,
    /// a代表底部元素, b代表父元素, fn(a,b)返回true时, 底部元素上滤
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T> {
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            values: vec![],
            comparator,
        }
    }

    /// 添加元素
    pub fn add(&mut self, value: T) {
        self.values.push(value);
        self.up_heapify(self.len() - 1);
    }

    /// 弹出堆顶元素
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let value = Some(self.values.swap_remove(0));
        self.down_heapify(0);
        value
    }

    // 上滤
    fn up_heapify(&mut self, idx: usize) {
        let mut idx = idx;

        while idx > 0 {
            let pdx = (idx - 1) >> 1;
            if (self.comparator)(&self.values[idx], &self.values[pdx]) {
                self.values.swap(idx, pdx);
                idx = pdx;
            } else {
                break;
            }
        }
    }

    // 下滤
    fn down_heapify(&mut self, idx: usize) {
        let mut idx = idx;

        while idx < self.len() {
            let ldx = 2 * idx + 1;
            let rdx = 2 * idx + 2;
            let mut cdx = idx;

            if ldx < self.len() && (self.comparator)(&self.values[ldx], &self.values[cdx]) {
                cdx = ldx;
            }
            if rdx < self.len() && (self.comparator)(&self.values[rdx], &self.values[cdx]) {
                cdx = rdx;
            }

            if cdx == idx {
                break;
            } else {
                self.values.swap(idx, cdx);
                idx = cdx;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.values.iter()
    }
}

impl<T> Heap<T>
where
    T: Ord,
{
    /// 小顶堆
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// 大顶堆
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut heap = Heap::new_min();
        heap.add(5);
        heap.add(3);
        heap.add(1);
        heap.add(4);
        heap.add(2);

        let mut iter = heap.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&4));
    }

    #[test]
    fn test_pop() {
        let mut heap = Heap::new_min();
        heap.add(5);
        heap.add(3);
        heap.add(1);
        heap.add(4);
        heap.add(2);

        assert_eq!(heap.pop(), Some(1));
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(3));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(5));
    }
}
