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
        //TODO
        /* 元素首先加到末尾，然后层层比较上浮，最终位于最终位置
         */
        self.items.push(value);
        self.count += 1;
        let mut idx = self.count;   // 根从1开始，所以元素的数量就是末尾的索引

        // 上浮
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            // 调用比较函数
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
                idx = parent_idx;
            }
            else {
                // 找到了正确的位置，提前返回
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

    fn smallest_child_idx(&self, idx: usize) -> usize {
        /* 获取idx节点的子节点中，最大或最小的节点的索引
         */
		let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        // 如果没有右节点，则左节点就是我们需要的，因为只有一个子节点
        if right > self.count {
            left
        }
        else {
            if (self.comparator)(&self.items[left], &self.items[right]) {
                left
            }
            else {
                right
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

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        /* 
         * 取出堆顶元素（idx为1）
         * 将最后一个元素移到堆顶
         * 执行下沉操作，满足堆的性质
         */
        if self.count == 0 {
            return None;
        }
		
        // 取出堆顶元素
        // 与 remove 不同, swap_remove 将取出指定元素并使用最后一个元素取代它, 而不是移动后续所有元素
        // 有更高的性能, 平均 O(1)
        let top = self.items.swap_remove(1);
        self.count -= 1;

        if self.count > 0 {
            // 下沉操作从堆顶开始
            let mut idx = 1;
            // 如果存在子节点
            while self.children_present(idx) {
                // 获取我们需要的最大或最小子节点
                let child_idx = self.smallest_child_idx(idx);
                // 如果符合要求就下沉
                // 如果不满足说明现在的位置就是正确的位置
                if !(self.comparator)(&self.items[idx], &self.items[child_idx]) {
                    self.items.swap(idx, child_idx);
                    idx = child_idx;
                }
                else {
                    break;
                }
            }
        }
        Some(top)
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