# 内存排序

## 重排和优化

## 内存模型

## Happens-Before

在 Rust 中，std::sync::atomic 模块提供了对原子操作的支持，这些操作保证在多线程环境中对共享数据的访问是安全的。Ordering 枚举是这个模块中非
常重要的一部分，它定义了原子操作的内存顺序保证级别。不同的内存顺序级别提供了不同的性能和安全性的权衡。

Ordering 枚举包含以下几种不同的内存顺序：

1. SeqCst (Sequential Consistency)：提供最强的内存顺序保证，确保所有线程看到的原子操作顺序与代码中的顺序一致。
2. Acquire：确保当前原子操作在所有后续的普通内存操作之前发生。 
3. Release：确保当前原子操作在所有之前的普通内存操作之后发生。 
4. AcqRel：结合了 Acquire 和 Release 的效果，即当前操作既是 Acquire 也是 Release。 
5. Relaxed：不提供任何内存顺序保证，只保证操作的原子性。 
6. Consume：与 Acquire 类似，但不具有对其他线程的可见性，仅用于优化。

这些内存顺序级别用于控制原子操作的执行顺序，以确保在多线程环境中的正确性和性能。例如，Acquire 和 Release 可以用来实现锁的获取和释放，而
Relaxed 则用于不需要顺序保证的场景，以获得更好的性能。

使用不同的 Ordering 级别可以帮助编译器和硬件优化代码，但同时也需要开发者对内存模型有深入的理解，以避免引入数据竞争或其他并发问题。