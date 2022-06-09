#[derive(Debug)]

//変数を構造化することで、意味づけする
struct  Rectangle {
    width: u32,
    height: u32
}

//構造体上にメソッドを作る
//selfは構造体を表す
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}


fn main() {
    //関連性のある変数をタプルにする
    let rect1 = Rectangle{width:30,height:50};
    let rect2 = Rectangle{width:10,height:40};
    let rect3 = Rectangle{width:60,height:45};
    
    println!("rect1にrect2はハマり込む? {}", rect1.can_hold(&rect2));
    println!("rect1にrect2はハマり込む? {}", rect1.can_hold(&rect3));

    //関連関数
    let sq = Rectangle::square(3);
    rect1.can_hold(&sq);
}
