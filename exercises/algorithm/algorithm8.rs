/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct MyStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        // 压入元素时，优先压入主队列
        // 哪个队列不为空，哪个队列就是主队列
        // 如果都为空则压入一号队列
        // pop()操作保证正常情况下，两个队列，至少有一个是空的
        if !self.q1.is_empty() {
            self.q1.enqueue(elem);
        }
        else if !self.q2.is_empty() {
            self.q2.enqueue(elem);
        }
        else {
            self.q1.enqueue(elem);
        }
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        // 如果栈为空
        if self.is_empty() {
            return Err("Stack is empty");
        }
		// 确定主队列
        let (full, empty) = if !self.q1.is_empty() {
            (&mut self.q1, &mut self.q2)
        } else {
            (&mut self.q2, &mut self.q1)
        };
        // 把主队列除了最后一个元素外的其他元素移入空队列，则最后一个元素为要出栈的
        // 出栈后，原来的主队列为空，变为辅助队列
        while full.size() > 1 {
            if let Ok(val) = full.dequeue() {
                empty.enqueue(val);
            }
        }
        // 弹出主队列的最后一个元素
        full.dequeue()
    }
    
    pub fn is_empty(&self) -> bool {
		//TODO
        // 当两个队列都为空时，栈为空
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = MyStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}