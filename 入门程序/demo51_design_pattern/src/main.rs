/*
状态模式（state pattern）是一个面向对象设计模式。
该模式的关键在于定义一系列值的内含状态。这些状态体现为一系列的 状态对象，同时值的行为随着其内部状态而改变。
我们将编写一个博客发布结构体的例子，它拥有一个包含其状态的字段，这是一个有着 "draft"、"review" 或 "published" 的状态对象.
*/

use demo51_design_pattern::{Post};

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}