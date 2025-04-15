//存储用户账户信息的结构体
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//匿名字段元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//没有任何字段的类单元结构体
struct EmptyStruct;

//生命周期确保结构体引用的数据有效性跟结构体本身保持一致。如果你尝试在结构体中存储一个引用,而不指定生命周期将是无效的
/*
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
*/

fn main() {
    //创建User结构体的实例
    let mut user1 = User {
        active:true,
        username:String::from("admin"),
        email:String::from("some@example.com"),
        sign_in_count:1,
    };
    //改变结构体实例的字段值
    user1.email = String::from("another@example.com");
    println!("The user1's email is: {}", user1.email);

    //不使用结构体更新语法从其他实例创建实例
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("anno@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    //使用结构体更新语法从其他实例创建实例
    let user3 = User {
        email: String::from("someone@example.com"),
        ..user2
    };

    //使用没有命名字段的元组结构体来创建不同的类型
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("The black color is: {}, {}, {}", black.0, black.1, black.2);
    println!("The origin point is: {}, {}, {}", origin.0, origin.1, origin.2);

    //使用类单元结构体
    let empty = EmptyStruct;
}

//build_user 函数获取 email 和用户名并返回 User 实例
fn build_user1(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

/*
使用字段初始化简写语法
如果结构体的字段与变量同名，可以使用简写语法来初始化结构体
*/
fn build_user2(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}