use std::io;

fn main() {
    loop {
        println!("欢迎使用我们的多功能程序，请选择以下功能：");
        println!("1. 摄氏度与华氏度转换");
        println!("2. 斐波那契数列查询");
        println!("0. 退出程序");
        let mut function_number = String::new();

        println!("请输入您的选择：");
        io::stdin()
            .read_line(&mut function_number)
            .expect("Failed to read line");

        let function_number: u32 = match function_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("输入无效，请输入一个有效的数字。");
                continue;
            }
        };

        match function_number {
            1 => {
                println!("您选择了摄氏度与华氏度转换功能。");
                t_conv();
            }
            2 => {
                println!("您选择了斐波那契数列查询功能。");
                fibonacci();
            }
            0 => {
                println!("程序退出。感谢您的使用！");
                break;
            }
            _ => {
                println!("输入无效，请输入1进行温度转换，2查询斐波那契数列，或0退出程序。");
            }
        }
    }
}

fn t_conv() {
    let mut mode = String::new();
    println!("摄氏度转华氏度请按1");
    println!("华氏度转摄氏度请按2");

    io::stdin()
        .read_line(&mut mode)
        .expect("Failed to read line");

    let mode: u32 = match mode.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("Error parsing number: {}", e);
            return;
        }
    };

    println!("请输入温度：");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: u32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("Error parsing number: {}", e);
            return;
        }
    };

    match mode {
        1 => {
            // 摄氏度转华氏度
            println!("对应华氏温度为：{}", (temp * 9 / 5 + 32));
        }
        2 => {
            // 华氏度转摄氏度
            println!("对应摄氏温度为：{}", ((temp - 32) * 5 / 9));
        }
        _ => {
            println!("无效的模式选择。");
        }
    }
}

fn fibonacci() {
    println!("请输入你想要查询的斐波那契数的序号：");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index = match index.trim().parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            println!("输入无效，请输入一个正整数。");
            return;
        }
    };

    if index == 0 {
        println!("斐波那契数列的第一项是0。");
    } else {
        let fib = match index {
            1 => 0,
            2 => 1,
            _ => {
                let mut fib_sequence = [0, 1];
                for _ in 2..index {
                    let next_fib = fib_sequence[0] + fib_sequence[1];
                    fib_sequence[0] = fib_sequence[1];
                    fib_sequence[1] = next_fib;
                }
                fib_sequence[1]
            }
        };
        println!("斐波那契数列的第{}项是：{}", index, fib);
    }
}
