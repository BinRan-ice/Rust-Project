/*
规则：

从 crate 根节点开始: 
 当编译一个 crate, 编译器首先在 crate 根文件
 （通常，对于一个库 crate 而言是src/lib.rs，对于一个二进制 crate 而言是src/main.rs）中寻找需要被编译的代码。

声明模块: 在 crate 根文件中，你可以声明一个新模块；
 比如，你用mod garden;声明了一个叫做garden的模块。编译器会在下列路径中寻找模块代码：
    1.内联，在大括号中，当mod garden后方不是一个分号而是一个大括号
    2.在文件 src/garden.rs
    3.在文件 src/garden/mod.rs

声明子模块:
 在除了 crate 根节点以外的其他文件中，你可以定义子模块。
 比如，你可能在src/garden.rs中定义了mod vegetables;。编译器会在以父模块命名的目录中寻找子模块代码：
    1.内联，在大括号中，当mod vegetables后方不是一个分号而是一个大括号
    2.在文件 src/garden/vegetables.rs
    3.在文件 src/garden/vegetables/mod.rs

模块中的代码路径: 
 一旦一个模块是你 crate 的一部分，你可以在隐私规则允许的前提下，从同一个 crate 内的任意地方，
 通过代码路径引用该模块的代码。举例而言，一个 garden vegetables 模块下的Asparagus类型可以在
 crate::garden::vegetables::Asparagus被找到。

私有 vs 公用: 
 一个模块里的代码默认对其父模块私有。为了使一个模块公用，应当在声明时使用pub mod替代mod。
 为了使一个公用模块内部的成员公用，应当在声明前使用pub。

use 关键字: 
 在一个作用域内，use关键字创建了一个成员的快捷方式，用来减少长路径的重复。
 在任何可以引用crate::garden::vegetables::Asparagus的作用域，
 你可以通过 use crate::garden::vegetables::Asparagus;创建一个快捷方式，然后你就可以在作用域中只写Asparagus来使用该类型。

*/
pub mod garden; //告诉编译器应该包含在src/garden.rs文件中发现的代码

use crate::garden::vegetables::Asparagus;   //一个 garden vegetables 模块下的Asparagus类型可以在这里找到

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!")
}
