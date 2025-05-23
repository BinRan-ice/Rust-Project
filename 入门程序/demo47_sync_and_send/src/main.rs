/*
通过 Send 允许在线程间转移所有权
Send 标记 trait 表明实现了 Send 的类型值的所有权可以在线程间传送。几乎所有的 Rust 类型都是Send 的，不过有一些例外，包括 Rc<T>：这是不能 Send 的，因为如果克隆了 Rc<T> 的值并尝试将克隆的所有权转移到另一个线程，这两个线程都可能同时更新引用计数。为此，Rc<T> 被实现为用于单线程场景，这时不需要为拥有线程安全的引用计数而付出性能代价。
因此，Rust 类型系统和 trait bound 确保永远也不会意外的将不安全的 Rc<T> 在线程间发送。当尝试在示例 16-14 中这么做的时候，会得到错误 the trait Send is not implemented for Rc<Mutex<i32>>。而使用标记为 Send 的 Arc<T> 时，就没有问题了。
任何完全由 Send 的类型组成的类型也会自动被标记为 Send。几乎所有基本类型都是 Send 的，除了第二十章将会讨论的裸指针（raw pointer）。

Sync 允许多线程访问
Sync 标记 trait 表明一个实现了 Sync 的类型可以安全的在多个线程中拥有其值的引用。换一种方式来说，对于任意类型 T，如果 &T（T 的不可变引用）是 Send 的话 T 就是 Sync 的，这意味着其引用就可以安全的发送到另一个线程。类似于 Send 的情况，基本类型是 Sync 的，完全由 Sync 的类型组成的类型也是 Sync 的。
智能指针 Rc<T> 也不是 Sync 的，出于其不是 Send 相同的原因。RefCell<T>（第十五章讨论过）和 Cell<T> 系列类型不是 Sync 的。RefCell<T> 在运行时所进行的借用检查也不是线程安全的。Mutex<T> 是 Sync 的，正如 “在线程间共享 Mutex<T>” 部分所讲的它可以被用来在多线程中共享访问。

手动实现 Send 和 Sync 是不安全的
通常并不需要手动实现 Send 和 Sync trait，因为由 Send 和 Sync 的类型组成的类型，自动就是 Send 和 Sync 的。因为它们是标记 trait，甚至都不需要实现任何方法。它们只是用来加强并发相关的不可变性的。
手动实现这些标记 trait 涉及到编写不安全的 Rust 代码，第十九章将会讲述具体的方法；当前重要的是，在创建新的由不是 Send 和 Sync 的部分构成的并发类型时需要多加小心，以确保维持其安全保证。“The Rustonomicon” 中有更多关于这些保证以及如何维持它们的信息。
*/

fn main() {
    println!("Hello, world!");
}
