//定义一个公有 Post 结构体来存放一些文本
pub struct Post {
    state: Option<Box<dyn State>>,  //Post 将在私有字段 state 中存放一个 Option<T> 类型的 trait 对象 Box<dyn State>
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),  //Post 的 new 函数会返回一个新的 Post 实例，它的 state 字段会被设置为 Some(Draft)
            content: String::new(),
        }
    }

    //存放博文内容的文本
    pub fn add_text(&mut self, text: &str) {    //实现方法 add_text 来向博文的 content 增加文本
        self.content.push_str(text);
    }

    // content 根据 Post 的当前状态返回值，所以需要 Post 代理一个定义于 state 上的 content 方法
    pub fn content(&self) -> &str { //更新 Post 的 content 方法来委托调用 State 的 content 方法
        self.state.as_ref().unwrap().content(self)
    }

    //请求审核博文来改变其状态
    pub fn request_review(&mut self) {  //
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    //增加改变 content 行为的 approve 方法
    pub fn approve(&mut self) { //将 state 设置为审核通过时应处于的状态
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

//定义了所有不同状态的博文所共享的行为
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

//博文的初始状态
struct Draft {}

impl State for Draft {
    //用来代表博文处于等待审核状态
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    //Draft 调用 approve 方法，并没有任何效果，因为审核通过后无需改变
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

//博文处于等待审核状态
struct PendingReview {}

impl State for PendingReview {
    //结构体 PendingReview 同样也实现了 request_review 方法，不过它不进行任何状态转换。相反它返回自身，因为当我们请求审核一个已经处于 PendingReview 状态的博文，它应该继续保持 PendingReview 状态。
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    //当对 PendingReview 调用 approve 时，它返回一个新的、装箱的 Published 结构体的实例。
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

//博文已经发布
struct Published {}

impl State for Published {
    //Published 结构体实现了 request_review 方法，不过它不进行任何状态转换。相反它返回自身，因为当我们请求审核一个已经处于 Published 状态的博文，它应该继续保持 Published 状态。
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    //Published 结构体实现了 approve 方法，不过它不进行任何状态转换。相反它返回自身，因为当我们请求审核一个已经处于 Published 状态的博文，它应该继续保持 Published 状态。
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    //Published 结构体会覆盖 content 方法并会返回 post.content 的值
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}