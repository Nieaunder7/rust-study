
 use std::io;
 use rand::Rng;
 use std::cmp::Ordering;

 fn main() {
    println!("숫자를 예측하세요!");  // 느낌표가 약간 괴롭다..
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    
    loop {
        // let mut guess = String::new();
        // 변경 안할거라서 mut이 없음, 정적 타입명시
        // rand::Rng 인데 rand는 어디서 온거지??
        println!("당신의 예측 숫자을 입력해주세요:");
        let mut guess = String::new();        
        
        let _ = io::stdin().read_line(&mut guess).expect("값을 읽는데 실패했습니다!");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // 예외처리를 바로 붙여서 처리하는게 깔끔해보이는 느낌
        // &mut 해서 붙이는게 주소를 직접 만지는 느낌이 나게 해주는듯..?
        // 근데... 읽는데 실패했습니다 라는 메시지가 안나온다. 왜지?

        println!("당신의 예측 숫자는: {guess}");

        // 출력 변수처리도 예쁜듯
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}
