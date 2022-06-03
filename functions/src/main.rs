fn main() {
    another_function(3);
    
    let num = five();
    println!("retun.No:{}", num);

    let add = plus_one(num);
    println!("add.num:{}", add)
}

/*関数リスト */

//戻り値無し
fn another_function(x: i32) {
    let y = {
        let z = 3;
        x + z
    };

    println!("Another Function:{}", y);
}

//戻り値有り
fn five() -> i32 {
    5
}

fn plus_one(num: i32) -> i32 {
    num + 1
}
