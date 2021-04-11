use proconio::input;

fn main() {
    input! {n: u32}
    let n_str: String = n.to_string();
    let digits: u32 = n_str.len() as u32;

    let fix: u32 = 10u32.pow((digits - 1) / 2) - 1;
    if digits % 2 == 0 {
        let check: u32 = digits / 2;
        let mut add: u32 = 0;
        for i in 0..check {
            let n: u32 = n_str.chars().nth((i) as usize).unwrap().to_digit(10).unwrap(); // 対象の桁の数字

            if i == check - 1 {
                add += n;
                // ここにロジック追加せねば。もらった数と対称な数を比較して、対称な数が小さければ、インクリメントする

            } else {
                if n == 0 {
                    continue;
                } else {
                    add += (10u32.pow(check) - 1) * (n - 1);
                }
            }
        }
        println!("{}", fix + add);
    } else {
        println!("{}", fix);
    }
}

