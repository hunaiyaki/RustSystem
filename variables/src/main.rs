use std::io;
fn main() {
        
    /*数値演算*/

        //加算
        let sum = 5 + 10;
        println!("{}",sum);

        //引き算
        let difference = 95.5 - 4.3;
        println!("{}", difference);

        //掛け算
        let product = 4 * 30;
        println!("{}", product);

        //割り算
        let quotient = 56.7 / 32.2;
        let floored = 2 / 3;
        println!("{} | {}", quotient,floored);

        //余り
        let remainder = 43 % 5;
        println!("{}", remainder);

    /*論理型*/

        let t = true;
        let f: bool = false;
        println!("{}",t);
        println!("{}",f);

    /*文字型*/
        let c = 'c';
        let z = 'ℤ';
        let heart_eyed_cat = '😻';
        println!("{}|{}|{}", c, z, heart_eyed_cat);
    
    /*複合型 */
        //タプル型
        let tup: (i32,f64,char) = (500,6.4, 'あ');
        println!("{}", tup.0);
        
    /*配列型 */
        let a: [i64; 5] = [1, 2, 3, 4, 5];
        println!("{}", a[0]);
        let a = [3; 5];
        println!("{}{}",a[0], a[4]);
    
    println!("");
    
    /*指定要素へのアクセス */
        let arr = ['１', '２', '３', '４', '５'];
        
        let mut index = String::new();


            println!("配列の何番目にアクセスしますか？");
            
            io::stdin()
                .read_line(&mut index)
                .expect("読み込み不可");
            
            let index: usize =index
                .trim().parse()
                .expect("入力された値は数値ではありません");

            let element = arr[index];
            println!("{}番目の値は{}です", index, element);
            
        }
