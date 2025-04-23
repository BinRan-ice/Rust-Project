/*
集成测试
为了编写集成测试，需要在项目根目录创建一个 tests 目录，与 src 同级。
*/
use demo32_test::add_two;   //导入 crate

mod common;

//一个 demo32_test crate 中函数的集成测试
#[test]
fn test_adds_two() {
    //一旦拥有了 tests/common/mod.rs，就可以将其作为模块以便在任何集成测试文件中使用。这里是一个 tests/integration_test.rs 中调用 setup 函数的 it_adds_two 测试的例子
    common::setup();
    assert_eq!(4, add_two(2));
}

/*
//指定测试函数的名称作为 cargo test 的参数来运行特定集成测试
cargo test --test integration_test  //运行特定集成测试,这个命令只运行了 tests 目录中我们指定的文件 integration_test.rs 中的测试。
*/