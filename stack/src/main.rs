
#[derive(Debug)]
struct Stack<T>{
    size: usize,
    data: Vec<T>
}

// 简易的栈实现
impl<T> Stack<T>{
    fn new() -> Stack<T>{
        Stack { size: 0, data: vec!() }
    }

    fn push(&mut self, element: T)
    {
        self.push(element);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T>
    {
        if self.size == 0 { return None; }

        self.size -= 1;

        self.data.pop()
    }

    fn peek(&self) -> Option<&T>{
        if self.size == 0 {
             return None;
        }
        
        self.data.get(self.size - 1)
    }

    fn is_empty(&self) -> bool{
        self.size == 0
    }

    // 当使用 &mut self 时,
    // println!("pop {:?}, size {}",s.peek().unwrap(), s.size());  这行会报错
    // QA: https://course.rs/basic/ownership/borrowing.html ： 同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
    fn size(&mut self)-> usize{
        self.size
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test] 
    fn stackPushTest(){
        let mut s = Stack::new();
        s.push(3);

        assert_eq!(&3, s.peek().unwrap());
        assert_eq!(3, s.pop().unwrap());
    }
}


fn main() {
    let mut s = Stack::new();
    s.push(1); 
    s.push(2);
    s.push(4);

    // 使用 unwrap 取出 option 的 some() 值 data ,  为 none 抛错

    println!("pop {:?}, size {}",s.peek().unwrap(), s.size()); 
    println!("top {:?}",s.peek().unwrap());
    println!(" size {}", s.size());

    println!("pop {:?}, size {}",s.pop().unwrap(), s.size());
    println!("is_empty:{}, stack:{:?}", s.is_empty(), s);
}
