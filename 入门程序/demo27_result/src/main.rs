use std::fs::File;
use std::io::ErrorKind;

/*
用 Result 处理可恢复的错误
enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

fn main() {
    let greeting_file_result = File::open("hello.txt"); //File::open 的返回值是 Result<T, E>。T 是成功时返回的值的类型，而 E 是失败时返回的错误的类型
    
    //使用 match 表达式处理可能会返回的 Result 成员
    /*
    let greeting_file = match greeting_file_result {
        Ok(file) => {
            println!("File opened successfully: {:?}", file);
        }
        Err(error) => {
            panic!("Error opening file: {:?}", error);
        }
    };
     */

    //使用不同的方式处理不同类型的错误
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {:?}", e),
            },
            other_error => panic!("Error opening file: {:?}", other_error),
        },
    };

    //失败时 panic 的简写：unwrap 和 expect

    /*
    Result<T, E> 类型定义了很多辅助方法来处理各种情况。
    其中之一叫做 unwrap。
    如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。如果 Result 是成员 Err，unwrap 会为我们调用 panic!。  
     */
    let greeting_file = File::open("hellos.txt").unwrap(); //如果 File::open 返回 Err，unwrap 会调用 panic!

    //使用 expect 而不是 unwrap 并提供一个好的错误信息可以表明你的意图并更易于追踪 panic 的根源。
    let greeting_file = File::open("hellos.txt").expect("Failed to open hellos.txt"); //如果 File::open 返回 Err，expect 会调用 panic! 并显示你提供的错误信息。
}
