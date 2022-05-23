### #Description
实现一个简易的 栈 结构

### #遇到问题


 在定义函数 size 是，可以使用 &mut self  | &self ， 对应着 可变应用 和 不可变引用

```Rust
    fn size(&mut self)-> usize{
        self.size
    }

    fn peek(&self) -> Option<&T>{
        if self.size == 0 {
             return None;
        }
        
        self.data.get(self.size - 1)
    }

```

当使用可变引用时 ：

```Rust
println!("pop {:?}, size {}",s.peek().unwrap(), s.size());  // 这行将报错
```

原因：  同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用

### References
[引用与借用](https://course.rs/basic/ownership/borrowing.html) ： https://course.rs/basic/ownership/borrowing.html 