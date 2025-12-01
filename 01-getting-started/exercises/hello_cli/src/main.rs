use std::io;

fn main() {
    println!("이름을 입력하세요:");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("입력을 읽는 데 실패했습니다");

    let name = name.trim();

    println!("안녕하세요, {}님!", name);
}
