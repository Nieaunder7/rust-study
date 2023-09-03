
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
   println!("숫자를 예측하세요!");  // 느낌표가 약간 괴롭다..
   let secret_number = rand::thread_rng().gen_range(1..=100);
//    println!("The secret number is: {secret_number}");
   let mut _try_cnt: u32 = 0;
   loop {
       println!("당신의 예측 숫자을 입력해주세요:");
       let mut guess = String::new();               
       let _ = io::stdin().read_line(&mut guess).expect("값을 읽는데 실패했습니다!");
       let guess: u32 = match guess.trim().parse() {
           Ok(num) => num,
           Err(_) => continue,
       };
       println!("당신의 예측 숫자는: {guess}");
       match guess.cmp(&secret_number) {
           Ordering::Less => println!("Too small!"),
           Ordering::Greater => println!("Too big!"),
           Ordering::Equal => {
               println!("You win! {}번 만에 맞췄습니다!", _try_cnt);
               break;
           },
       }
       _try_cnt += 1;
   }
}
