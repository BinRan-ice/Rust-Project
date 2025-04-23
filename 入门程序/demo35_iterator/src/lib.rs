#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    /*
    shoes_in_size 函数接受一个 Vec<Shoe> 类型的集合和一个鞋码 shoe_size。
    功能：过滤出所有鞋码为 shoe_size 的鞋子，并将结果收集到一个新的 Vec<Shoe> 中返回。
    into_iter：将 shoes 转换为一个迭代器。
    filter：只保留满足 s.size == shoe_size 条件的元素。
    collect：将过滤后的元素收集到一个新的 Vec<Shoe>。
     */
    shoes.into_iter().filter(|s| s.size == shoe_size).collect() //使用 filter 和一个捕获环境中变量 shoe_size 的闭包来遍历一个 Shoe 结构体集合。它只会返回指定鞋码的鞋子。
}

#[cfg(test)]    
mod tests {
    use std::alloc::LayoutErr;

    use super::*;

    #[test] //在迭代器上（直接）调用 next 方法
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();  //调用 iter 方法创建一个迭代器
        assert_eq!(v1_iter.next(), Some(&1));  //每次调用 next 时，迭代器会返回 Some(&value)，其中 value 是迭代器上的引用
        assert_eq!(v1_iter.next(), Some(&2));  
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);  //当 next 返回 None 时，迭代器结束
    }

    #[test] //调用 sum 方法获取迭代器所有项的总和
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();  //sum 方法是一个消费适配器，它获取迭代器的所有权并调用 next 直到迭代器返回 None
        assert_eq!(total, 6);
    }   //调用 sum 之后不再允许使用 v1_iter 因为调用 sum 时它会获取迭代器的所有权。

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}