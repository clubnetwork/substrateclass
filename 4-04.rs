// 04 为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同，可以上传代码片段，或者代码的链接；
fn main() {
    let red = TrafficLight::Red;
    println!("Red Light is turned on: {} sec.", red.get_time());

    let yellow = TrafficLight::Yellow;
    println!("Yellow Light is turned on: {} sec.", yellow.get_time());

    let green = TrafficLight::Green;
    println!("Green Light is turned on: {} sec.", green.get_time());
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub trait LightTime{
    fn get_time(&self) -> u8;
}

impl LightTime for TrafficLight {
    fn get_time(&self) -> u8 {
        match &self {
            TrafficLight::Red => 40,
            TrafficLight::Yellow => 20,
            TrafficLight::Green => 60,
        }
    }
}
