/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;


pub struct Heap<T>
where
    T: Default+Clone,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default+Clone,
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
        //TODO
        self.items.push(value);
        self.count+=1;
        self.sift_up(self.count);
        
    }

    fn sift_up(&mut self,mut idx: usize){
        while idx>1{
            let parent_idx=self.parent_idx(idx);
            if (self.comparator)(&self.items[idx],&self.items[parent_idx]){
                self.items.swap(idx,parent_idx);
                idx=parent_idx;
            }else{
                break;
            }
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

    fn smallest_child_idx(&self, mut idx: usize) -> usize {
        //TODO
		let left=self.left_child_idx(idx);
        let right=self.right_child_idx(idx);
        if !(self.comparator)(&self.items[left],&self.items[right]){
            right
        }else{
            left
        }
    }
    fn pop(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        let result = self.items[1].clone(); // 堆顶元素在索引1
        if self.count == 1 {
            // 如果堆中只剩下一个元素，直接移除并返回
            self.count -= 1;
            self.items.pop();
        } else {
            // 将堆中的最后一个元素移动到根位置
            self.items[1] = self.items[self.count - 1].clone();
            self.count -= 1;
            // 移除原来的最后一个元素
            self.items.pop();
            // 对新的根元素执行下沉操作以维护堆的性质
            self.sift_down(1);
        }
        Some(result)
    }
    
    fn sift_down(&mut self, mut idx: usize) {
        let mut smallest = idx;
        let mut left = self.left_child_idx(idx);
        let mut right = self.right_child_idx(idx);
    
        loop {
            if left < self.count {
                if right < self.count {
                    // 左右子节点都存在
                    if (self.comparator)(&self.items[left], &self.items[right]) {
                        smallest = right;
                    } else {
                        smallest = left;
                    }
                } else {
                    // 只有左子节点存在
                    smallest = left;
                }
            } else {
                break; // 没有子节点，结束下沉
            }
    
            if !(self.comparator)(&self.items[smallest], &self.items[idx]) {
                break; // 当前节点已经满足堆的性质，结束下沉
            }
    
            self.items.swap(idx, smallest);
            idx = smallest;
            left = self.left_child_idx(idx);
            right = self.right_child_idx(idx);
        }
    }
    
}

impl<T> Heap<T>
where
    T: Default + Ord+Clone,
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

impl<T> Iterator for Heap<T>
where
    T: Default+Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		self.pop()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord+Clone,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord+Clone,
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
        assert_eq!(heap.next(), Some(4));
    }
}