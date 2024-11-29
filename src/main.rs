use std::io;

fn main() {
    loop {
        println!("选择你需要的功能：");
        let mut function_number = String::new();
        println!("输入1进行温度转换，输入0退出程序");

        io::stdin()
            .read_line(&mut function_number)
            .expect("Failed to read line");

        let function_number: u32 = match function_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("输入无效，请输入数字。");
                continue;
            }
        };

        match function_number {
            1 => {
                t_conv();
            }
            0 => {
                println!("程序退出。");
                break;
            }
            _ => {
                println!("输入无效，请输入1进行温度转换或0退出程序。");
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
