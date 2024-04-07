fn main() {
    let mut height_cm;
    let mut weight;
    loop {
        println!("身長(cm)は? ");
        height_cm = input_f(0.0);
        if height_cm > 0.0 {
            break;
        }
        println!("正しい数値を入力してください。")
    }
    loop {
        println!("体重(kg)は? ");
        weight = input_f(0.0);
        if weight > 0.0 {
            break;
        }
        println!("正しい数値を入力してください。")
    }

    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);
    println!("bmi={:.1}", bmi);
    if bmi < 18.5 {
        println!("低体重");
    } else if bmi < 25.0 {
        println!("普通体重");
    } else if bmi < 30.0 {
        println!("肥満1度");
    } else if bmi < 35.0 {
        println!("肥満2度");
    } else if bmi < 40.0 {
        println!("肥満3度");
    } else {
        println!("肥満4度");
    }
}

fn input_str() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("入力エラー");
    s.trim_end().to_string()
}

fn input_f(def: f64) -> f64 {
    let s = input_str();
    // MEMO: 関数の返り値から型推論がはたらき、String::parse<f64>が呼ばれている
    match s.trim().parse() {
        Ok(v) => v,
        Err(_) => def,
    }
}
