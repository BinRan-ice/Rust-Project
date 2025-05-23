pub trait  Draw {   //Draw trait 的定义
    fn draw(&self);
}

pub struct Screen { //一个 Screen 结构体的定义，它带有一个字段 components，其包含实现了 Draw trait 的 trait 对象的 vector
    pub components: Vec<Box<dyn Draw>>, 
}

impl Screen {
    pub fn run(&self) {  //Screen 结构体上的 run 方法，它会遍历 components 中的每一个元素并调用 draw 方法
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {  //一个实现了 Draw trait 的 Button 结构体
    fn draw(&self) {    //Button 结构体上的 draw 方法，它会打印出 Button 的信息
        println!("Drawing a button with width {}, height {}, and label '{}'", self.width, self.height, self.label);
    }
}