//AveragedCollection 结构体维护了一个整型列表及其所有元素的平均值。
pub struct AveragedCollection { //该结构体被标记为 pub，这样其他代码就可以使用它，但结构体内的字段保持私有
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {   //在 AveragedCollection 结构体上实现了 add、remove 和 average 公有方法
    pub fn add(&mut self, value: i32) {
        self.list.push(value); //将一个新值添加到 list 中
        self.update_average(); //更新 average 字段
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop(); //从 list 中移除最后一个值
        match result {
            Some(value) => {
                self.update_average(); //如果成功移除了一个值，就更新 average 字段
                Some(value)
            }
            None => None, //如果 list 是空的，返回 None
        }
    }

    pub fn average(&self) -> f64 {
        self.average //返回 average 字段的值
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum(); //计算 list 中所有元素的和
        self.average = total as f64 / self.list.len() as f64; //计算平均值
    }
}