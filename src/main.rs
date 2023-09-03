use std::io;

fn fib(term: u32, val: u32, prev: u32) -> u32 {
    if term == 0 {
        return prev;
    } else {
        return fib(term - 1, val+prev , val);
    }
}

fn main() {
    let mut guess = String::new();
    let _ = io::stdin()
        .read_line(&mut guess)
        .expect("값을 읽는데 실패했습니다!");
    let idx: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => return println!("0"),
    };
    println!("{}", fib(idx, 1, 0));
}
