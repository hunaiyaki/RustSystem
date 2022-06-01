use std::io;//入出力
use rand::Rng;//乱数
use std::cmp::Ordering;

fn main() {
    println!("数を当てて！");

    let secret_number = rand::thread_rng().gen_range(1..11);
    
    loop {
        println!("予想した数値を入力してください。");

        let mut guess= String::new();
   
        io::stdin()
            .read_line(&mut guess)
            .expect("読み込みに失敗しました。");

        let guess: u64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("数値を入力してください");
                continue;
            }        
        };

        println!("予想した値は:{}",guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎます"),
            Ordering::Greater => println!("大きすぎます"),
            Ordering::Equal => {println!("成功！");
                    break;
                }
        }
    }
}