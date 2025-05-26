/*
关联类型在 trait 定义中指定占位符类型
关联类型（associated types）让我们可以在 trait 里面增加一个待定义的类型（类型占位符），将类型占位符与 trait 相关联，
这样 trait 的方法签名中就可以使用这些占位符类型。trait 的实现者在实现这个 trait 的时候，会指定一个具体类型，来替换掉这个占位符。
*/

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Iterator for Counter { //Counter 结构体实现了 Iterator trait，其中指定 Item 的类型为 u32
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

/* trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
这些代码看来应该很熟悉，这是一个带有一个方法和一个关联类型的 trait。
比较陌生的部分是尖括号中的 Rhs=Self：这个语法叫做 默认类型参数（default type parameters）。
Rhs 是一个泛型类型参数（“right hand side” 的缩写），它用于定义 add 方法中的 rhs 参数。
如果实现 Add trait 时不指定 Rhs 的具体类型，Rhs 的类型将是默认的 Self 类型，也就是在其上实现 Add 的类型。 */

use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

//在 Millimeters 上实现 Add，以便能够将 Millimeters 与 Meters 相加
impl Add<Meters> for Millimeters {    //为 Millimeters 结构体实现 Add trait，其中指定 Rhs 的类型为 Meters
    type Output = Millimeters;    //定义关联类型 Output，用于指定 + 运算符的返回类型

    fn add(self, other: Meters) -> Millimeters {    //定义 add 方法
        Millimeters(self.0 + (other.0 * 1000))
    }
}