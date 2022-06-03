fn main() {
    
    /*if文 */
    let number = 3;
    
    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }

    if number != 0 {
        println!("0以外の何かです。");
    }

    let number2 = 6;

    if number2 % 4 == 0 {
        println!("数値は４で割り切れます。");
    } else if number2 % 3 ==0 {
        println!("数値は３で割り切れます。")
    } else if number2 % 2 == 0 {
        println!("数値は２で割り切れます・")
    } else {
        println!("数値は４、３、２で割り切れません。");
    }

    let condition = true;
    let number3 = if condition {5} else {6};

    println!("number3の値は{}", number3);
    
    /*ループ文 */
    //loop
    let mut count = 0;
    'counting_up: loop {
        println!("count= {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining= {}", remaining);
            if remaining == 9 {
                break;
            } else if  count == 2 {
                break 'counting_up;
            } else {
                remaining -= 1;
            }
        }
        count += 1;
    }
    println!("End count= {}", count);

    //while
    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    println!("LIFTOFF!!");

    //for
    let a = [10,20,30,40,50];

    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev(){
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");

}
