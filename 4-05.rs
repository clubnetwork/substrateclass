//05 实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None，可以上传代码片段，或者代码的链接；
fn main() {
    let v1: Vec<u32> = vec![150u32, 69u32];
    let s = sum(v1);
    match s {
        Some(_x) => println!("Sum result is: {}", s.unwrap()),
        None => println!("Overflow occurred!"),
    }
    
}

fn sum(vec: Vec<u32>) -> Option<u32> {
    let mut total: Option<u32> = Some(0);
    for num in vec {
        total = total.unwrap().checked_add(num);
    }
    total
}

