use trpl::{Html, Either};  //引入 trpl crate

/* //定义一个 async 函数来获取一个 HTML 页面的标题元素
async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}

//等待一个使用异步代码块的 `trpl::run`
fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
} */

//定义一个 async 函数来获取一个 HTML 页面的标题元素
async fn page_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;   //使用 `await` 关键字的链式调用
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

//等待一个使用异步代码块的 `trpl::run`
fn main() {
    //通过两个来自命令行的不同 URL 来调用 page_title 并使其相互竞争,并发代码
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) = 
            match trpl::race(title_fut_1, title_fut_2).await {  //race 返回了一个类型 trpl::Either,它使用 Left 和 Right 来表示 “一个或另一个”
                Either::Left(left) => left,
                Either::Right(right) => right,
            };
        
        println!("{url} returned first");

        match maybe_title {
            Some(title) => println!("Its page title is: {title}"),
            None => println!("It doesn't have a title"),
        }
    })
}