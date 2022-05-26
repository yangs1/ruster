### Description
添加一些工具的，简易版区块链

### 知识点

1. Rust 目录结构:  `examples` 存放项目 demo
   ```Rust
      cargo run --example filename   // 执行对应的文件
   ```

2. tracing 日志追踪 | chrono 时间工具 | ....
   tracing资料： https://zhuanlan.zhihu.com/p/496028010
   （看 docs.rs 好难，该怎么学 Rust 的 crate 库呢， 之后可能得重新学下）

3. 语法糖 ?

```Rust
  Ok(bincode::serialize(data)?) 
  ? 的含义相当于 ： ?操作符可以方便地对Result<T,E>进行值提取（Ok(v) => v），或者返回一个错误类型值，提前结束当前函数
  match result {
        Ok(v) => v,
        Err(e) => return Err(e.into())
  }
```

### References
[用Rust实现区块链](https://mp.weixin.qq.com/s?__biz=Mzg5MjA1ODYzNg==&mid=2247484460&idx=1&sn=b79b1051f40db383a2d2feb568cb3fe8&chksm=cfc2a94ff8b52059b2402785330133ce6a6734a3abcd3343c08154716acca5eb8369a4f4cd12&token=1912223334&lang=zh_CN#rd)