# 本项目用于评估jsi以及安全封装的性能开销，每条用例均执行1万次调用

## extend_safe使用trait实现继承以及父函数调用

## extend_unsafe使用裸指针实现继承以及父函数调用

## wrapper_with_jsi和wrapper_without_jsi对比了直接调用，以及通过wrapper调用的性能

wrapper_with_jsi涉及jsi跨语言

wrapper_without_jsi为纯rust调用

## 运行：cargo bench
