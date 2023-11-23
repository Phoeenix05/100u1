fn main() {
    let n: i64 = std::env::var("WN_VAR_N")
        .unwrap_or("100000".to_owned())
        .parse()
        .unwrap();

    for i in 0..n {
        let s: i64 = i
            .to_string()
            .chars()
            .map(|c| c.to_string().parse::<i64>().unwrap())
            .sum();

        let r: i64 = s
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse()
            .unwrap();

        if i == s * r {
            let t = s * r;
            dbg!(t);
        }
    }
}
