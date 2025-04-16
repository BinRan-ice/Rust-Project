mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // 使用super允许我们引用父模块中的已知项
    }

    fn cook_order() {}

    // 创建公有结构体
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    /*
    因为 back_of_house::Breakfast 具有私有字段
    所以这个结构体需要提供一个公共的关联函数来构造 Breakfast 的实例 (这里我们命名为 summer)
    如果 Breakfast 没有这样的函数，我们将无法在 eat_at_restaurant 中创建 Breakfast 实例
     */
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    //与之相反，如果我们将枚举设为公有，则它的所有成员都将变为公有
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

/*
以下代码使用cargo build编译时会报错：错误信息说 hosting 模块是私有的，需要将其改为公有的
修改后代码依旧报错：add_to_waitlist 函数是私有的，需要将其改为公有的
*/
pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    // 在夏天订购一个黑麦吐司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 试图更改这个早餐的面包种类
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果尝试更改这个早餐的季节水果，那么编译器会报错
    // 不允许查看或修改早餐附带的季节水果
    // meal.seasonal_fruit = String::from("blueberries");

    // 试图创建一个 Appetizer::Soup 枚举的实例
    let _order1 = back_of_house::Appetizer::Soup;
    // 试图创建一个 Appetizer::Salad 枚举的实例
    let _order2 = back_of_house::Appetizer::Salad;
}