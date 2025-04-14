/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.count += 1;
        if self.items.len() < self.count + 1 {//为什么不用capacity而用len
            self.items.push(value);
            println!("aa");
        }else
        {
            self.items[self.count] = value;//不能只用push   
            println!("bb");         
        }

        let mut present_idx = self.count;
        let mut parent_idx = self.parent_idx(self.count);
        let mut parent = &self.items[parent_idx];
        let mut present = &self.items[present_idx];
        while parent_idx > 0 && (self.comparator)(present, parent) {
            self.items.swap(parent_idx, present_idx);
            present_idx = parent_idx;
            parent_idx = self.parent_idx(present_idx);
            present = &self.items[present_idx];
            parent = &self.items[parent_idx];
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        if !self.children_present(idx) {
            10000
        }
        else if idx * 2 + 1 > self.count {
            self.left_child_idx(idx)
        }
        else
        {
            let child1 = &self.items[self.left_child_idx(idx)];
            let child2 = &self.items[self.right_child_idx(idx)];
            if (self.comparator)(child1, child2) {
                self.left_child_idx(idx) 
            } 
            else{
                self.right_child_idx(idx)               
            }

        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T: Clone> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count == 0 {
            return None
        }
        else
        {
            let ret = self.items[1].clone();
            self.items.swap(1, self.count);            
            self.count -= 1;
            let mut present_idx = 1;
            if !self.children_present(present_idx){
                return Some(ret);
            }
            let mut child_idx = self.smallest_child_idx(present_idx);
            let mut present = &self.items[present_idx];
            let mut child = &self.items[child_idx];
            while (self.comparator)(child, present) {
                self.items.swap(present_idx, child_idx);
                present_idx = child_idx;
                if !self.children_present(present_idx) {
                    break;
                }
                child_idx = self.smallest_child_idx(present_idx);
                present = &self.items[present_idx];
                child = &self.items[child_idx];
            }
            Some(ret)
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}