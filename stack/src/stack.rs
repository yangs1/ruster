// 简易栈 mod
pub mod stack {
    #[derive(Debug)]
    pub struct Stack<T> {
        size: usize,
        data: Vec<T>,
    }

    // 简易的栈实现
    impl<T> Stack<T> {
        pub fn new() -> Stack<T> {
            Stack {
                size: 0,
                data: vec![],
            }
        }

        pub fn push(&mut self, element: T) {
            self.data.push(element);
            self.size += 1;
        }

        pub fn pop(&mut self) -> Option<T> {
            if self.size == 0 {
                return None;
            }

            self.size -= 1;

            self.data.pop()
        }

        pub fn peek(&self) -> Option<&T> {
            if self.size == 0 {
                return None;
            }

            self.data.get(self.size - 1)
        }

        pub fn is_empty(&self) -> bool {
            self.size == 0
        }

        // 当使用 &mut self 时,
        // println!("pop {:?}, size {}",s.peek().unwrap(), s.size());  这行会报错
        // QA: https://course.rs/basic/ownership/borrowing.html ： 同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
        pub fn size(&self) -> usize {
            self.size
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn stack_push_test() {
            let mut s = Stack::new();
            s.push(3);

            assert_eq!(&3, s.peek().unwrap());
            assert_eq!(3, s.pop().unwrap());
        }
    }
}
