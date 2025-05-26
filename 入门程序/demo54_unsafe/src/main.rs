/*
不安全的超能力

可以通过 unsafe 关键字来切换到不安全 Rust，接着可以开启一个新的存放不安全代码的块。
这里有五类可以在不安全 Rust 中进行而不能用于安全 Rust 的操作，它们称之为 “不安全的超能力。（unsafe superpowers）” 这些超能力是：
（1）解引用裸指针
（2）调用不安全的函数或方法
（3）访问或修改可变静态变量
（4）实现不安全 trait
（5）访问 union 的字段
*/

/* fn main() {
/*
1.解引用裸指针
裸指针与引用和智能指针的区别在于
(1)允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
(2)不保证指向有效的内存
(3)允许为空
(4)不能实现任何自动清理功能
*/

    //如何从引用同时创建不可变和可变裸指针(可以在安全代码中创建裸指针，d但是不能在不安全块之外解引用裸指针)
    let mut num = 5;
    let r1 = &num as *const i32; //创建一个不可变的裸指针
    let r2 = &mut num as *mut i32; //创建一个可变的裸指针

    /* //创建指向任意内存地址的裸指针
    let address = 0x012345usize;
    let r = address as *const i32; */

    //在 unsafe 块中解引用裸指针
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

} */

/* unsafe fn dangerous() {}

fn main() {
/*
2.调用不安全函数或方法
第二类可以在不安全块中进行的操作是调用不安全函数。不安全函数和方法与常规函数方法十分类似，除了其开头有一个额外的 unsafe。
*/
    unsafe {
        dangerous();
    }
    //unsafe { dangerous(); } //调用不安全函数或方法
    //dangerous(); //error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
} */

/* //自定义split_at_mut函数的实现中使用不安全代码(如何使用 unsafe 块，裸指针和一些不安全函数调用来实现 split_at_mut)
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr(); //获取 slice 的裸指针

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid), //使用 from_raw_parts_mut 函数创建一个指向 slice 的裸指针的切片
            slice::from_raw_parts_mut(ptr.add(mid), len - mid) //使用 add 方法创建一个指向 mid 之后元素的裸指针
        ) 
    }
}

fn main() {
/*
2.1创建不安全代码的安全抽象
仅仅因为函数包含不安全代码并不意味着整个函数都需要标记为不安全的。事实上，将不安全代码封装进安全函数是一个常见的抽象。
*/
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3); //使用安全的 split_at_mut 函数
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
} */

/* extern "C" {    //使用 extern 关键字和一个 C 标记来调用外部 C 代码
    fn abs(input: i32) -> i32;
}

fn main() {
/*
2.2使用 extern 函数调用外部代码
有时你的 Rust 代码可能需要与其他语言编写的代码交互。
为此 Rust 有一个关键字，extern，有助于创建和使用 外部函数接口（Foreign Function Interface，FFI）。
外部函数接口是一个编程语言用以定义函数的方式，其允许不同（外部）编程语言调用这些函数。
*/ 
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
} */

/* //定义和使用一个不可变静态变量
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
/*
3.访问或修改可变静态变量
Rust 允许在全局作用域中声明全局变量，这些变量可以是可变的，但是访问和修改可变静态变量是不安全的。
*/
    println!("name is: {}", HELLO_WORLD);
    //HELLO_WORLD = "Hello, Rust!"; //error[E0133]: cannot assign to static item `HELLO_WORLD`

    add_to_count(3);
    unsafe {    //读取或修改一个可变静态变量是不安全的
        println!("COUNTER: {}", COUNTER);
    }
} */

/* //定义和使用一个不安全 trait
unsafe trait Foo {
    //为了实现这个 trait，必须是不安全的
}

unsafe impl Foo for i32 {
    //为了实现这个 trait，必须是不安全的
}

fn main() {
/*
4.实现不安全 trait
unsafe 的另一个操作用例是实现不安全 trait。
当 trait 中至少有一个方法中包含编译器无法验证的不变式（invariant）时 trait 是不安全的。
可以在 trait 之前增加 unsafe 关键字将 trait 声明为 unsafe，同时 trait 的实现也必须标记为 unsafe
*/
} */

fn main() {
/*
5.访问 union 的字段
仅适用于 unsafe 的最后一个操作是访问 联合体 中的字段，union 和 struct 类似，但是在一个实例中同时只能使用一个声明的字段。
联合体主要用于和 C 代码中的联合体交互。访问联合体的字段是不安全的，因为 Rust 无法保证当前存储在联合体实例中数据的类型。
*/
}