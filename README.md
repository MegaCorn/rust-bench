## 本项目用于评估jsi以及安全封装的性能开销，每条用例均执行1万次调用

### extend_safe使用trait实现继承以及父函数调用

### extend_unsafe使用裸指针实现继承以及父函数调用

### wrapper_ffi和wrapper_rust对比了直接调用，以及通过wrapper调用的性能

wrapper_ffi涉及跨语言

wrapper_rust为纯rust调用

### 运行：

本机运行(依赖cargo, 适用于linux和windows): cargo bench

手机运行: cargo build --release 编译可执行二进制, 通过hdc file send推送至手机
