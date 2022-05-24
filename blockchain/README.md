### Description
简单实现下区块链，模拟区块链的简易流程

### 知识点小结

1.   多目录结构下通过 `Cargo.toml` 定义 工作空间
```Rust
cargo new main
cargo new core --lib
cargo new utils --lib
```

2. rust-analyzer 在 `Cargo.toml` 解析失败时，会无法工作....

3. 泛型的约束可以用多种方式表达
```Rust
表达式 1：  pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8>
where T: Serialize{ }

表达式 2：  pub fn my_serialize<T: ?Sized + Serialize>(value: &T) -> Vec<u8> {}
```

4.  关于 `?Sized`  : https://zhuanlan.zhihu.com/p/21820917
```Rust
泛型约束中使用 Sized、!Sized、?Sized 三种写法。其中 T:Sized 代表类型必须是编译期确定大小的，T:!Sized 代表类型必须是编译期不确定大小的，T:?Sized 代表以上两种情况都可以。
```

### References
[Rust编程小项目：编写简单的区块链](https://www.bilibili.com/video/BV145411t7qp?p=7)