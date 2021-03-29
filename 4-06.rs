// 06 实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束，可以上传代码片段，或者代码的链接。
fn main() {
    let shape_1 = Triangle { bot_edge: 6.0, height: 9.0, };
    let shape_2 = Rectangle { width: 100.0, height: 8.0, };
    let shape_3 = Round { radius: 15.0 };
    print_area(&shape_1);
    print_area(&shape_2);
    print_area(&shape_3);
}

fn print_area<T: AreaCalculable>(shape: &T) {
    println!("The area is: {:?}", shape.area());
}

pub trait AreaCalculable {
    fn area(&self) -> f32;
}

pub struct Triangle {
    pub bot_edge: f32,
    pub height: f32,
}

impl AreaCalculable for Triangle {
    fn area(&self) -> f32 {
        self.bot_edge * self.height * 0.5
    }
}

pub struct Rectangle {
    pub width: f32,
    pub height: f32,
}

impl AreaCalculable for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

pub struct Round {
    pub radius: f32,
}

impl AreaCalculable for Round {
    fn area(&self) -> f32 {
        self.radius * self.radius * 3.1415926
    }
}
