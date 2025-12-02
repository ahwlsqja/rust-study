// ============================================================
// 실습: 온도 변환기
// ============================================================
// 섭씨(Celsius)와 화씨(Fahrenheit)를 상호 변환하는 프로그램
// 학습 내용: 변수, 함수, 조건문, 반복문, 사용자 입력
// ============================================================

use std::io;

// ============================================================
// 변환 공식:
// ┌─────────────────────────────────────────────────────────┐
// │  섭씨 -> 화씨: F = C × 9/5 + 32                         │
// │  화씨 -> 섭씨: C = (F - 32) × 5/9                       │
// │                                                         │
// │  예시:                                                  │
// │  0°C = 32°F (물의 어는점)                               │
// │  100°C = 212°F (물의 끓는점)                            │
// │  37°C = 98.6°F (체온)                                   │
// └─────────────────────────────────────────────────────────┘
// ============================================================

/// 섭씨를 화씨로 변환하는 함수
///
/// # 매개변수
/// * `celsius` - 섭씨 온도 (f64 타입)
///
/// # 반환값
/// * 화씨 온도 (f64 타입)
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    // 공식: F = C × 9/5 + 32
    // 또는: F = C × 1.8 + 32
    celsius * 9.0 / 5.0 + 32.0
}

/// 화씨를 섭씨로 변환하는 함수
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    // 공식: C = (F - 32) × 5/9
    (fahrenheit - 32.0) * 5.0 / 9.0
}

/// 사용자로부터 숫자를 입력받는 함수
///
/// # 반환값
/// * `Option<f64>` - 성공 시 Some(숫자), 실패 시 None
///
/// Option은 값이 있을 수도 있고 없을 수도 있음을 나타내는 열거형이다.
/// - Some(value): 값이 있음
/// - None: 값이 없음
/// 05-enums-pattern-matching에서 자세히 학습한다.
fn read_number() -> Option<f64> {
    let mut input = String::new();

    // 입력 받기
    io::stdin()
        .read_line(&mut input)
        .expect("입력 실패");

    // 문자열을 숫자로 변환 시도
    // trim(): 공백/줄바꿈 제거
    // parse(): 문자열을 숫자로 파싱
    // ok(): Result를 Option으로 변환 (에러면 None)
    input.trim().parse().ok()
}

/// 메뉴를 출력하는 함수
fn print_menu() {
    println!();
    println!("┌─────────────────────────────────┐");
    println!("│       온도 변환기               │");
    println!("├─────────────────────────────────┤");
    println!("│  1. 섭씨 -> 화씨 변환           │");
    println!("│  2. 화씨 -> 섭씨 변환           │");
    println!("│  3. 온도표 출력                 │");
    println!("│  4. 종료                        │");
    println!("└─────────────────────────────────┘");
    print!("선택: ");
}

/// 온도 변환표를 출력하는 함수
fn print_temperature_table() {
    println!();
    println!("┌────────────────────────────────────┐");
    println!("│         온도 변환표                │");
    println!("├──────────────┬─────────────────────┤");
    println!("│    섭씨(°C)  │     화씨(°F)        │");
    println!("├──────────────┼─────────────────────┤");

    // for 반복문으로 -40부터 100까지 20도 간격으로 출력
    // 범위에 step이 필요할 때는 (시작..끝).step_by(n) 사용
    // 또는 while 사용
    let mut celsius = -40;
    while celsius <= 100 {
        let fahrenheit = celsius_to_fahrenheit(celsius as f64);
        // {:>10.1} : 오른쪽 정렬, 10칸, 소수점 1자리
        println!("│{:>10}    │{:>15.1}      │", celsius, fahrenheit);
        celsius += 20;  // 20도씩 증가
    }

    println!("└──────────────┴─────────────────────┘");
}

fn main() {
    println!("====================================");
    println!("  온도 변환 프로그램에 오신 것을 환영합니다!");
    println!("====================================");

    // 메인 루프 - 사용자가 종료를 선택할 때까지 반복
    loop {
        print_menu();

        // 플러시하여 "선택: " 즉시 출력 (버퍼링 문제 방지)
        use std::io::Write;
        io::stdout().flush().unwrap();

        // 메뉴 선택 입력
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("입력 실패");

        let choice = choice.trim();

        // 메뉴에 따른 분기
        // match는 05장에서 배우므로, 여기서는 if-else 사용
        if choice == "1" {
            // 섭씨 -> 화씨
            println!("\n섭씨 온도를 입력하세요:");

            // if let: Option에서 값 추출 (05장에서 자세히)
            // 지금은 "값이 있으면 celsius에 넣고 실행"으로 이해
            if let Some(celsius) = read_number() {
                let fahrenheit = celsius_to_fahrenheit(celsius);
                println!();
                println!("  {:.2}°C = {:.2}°F", celsius, fahrenheit);
            } else {
                println!("잘못된 입력입니다. 숫자를 입력해주세요.");
            }

        } else if choice == "2" {
            // 화씨 -> 섭씨
            println!("\n화씨 온도를 입력하세요:");

            if let Some(fahrenheit) = read_number() {
                let celsius = fahrenheit_to_celsius(fahrenheit);
                println!();
                println!("  {:.2}°F = {:.2}°C", fahrenheit, celsius);
            } else {
                println!("잘못된 입력입니다. 숫자를 입력해주세요.");
            }

        } else if choice == "3" {
            // 온도표 출력
            print_temperature_table();

        } else if choice == "4" {
            // 종료
            println!("\n프로그램을 종료합니다. 안녕히 가세요!");
            break;  // loop 탈출

        } else {
            println!("잘못된 선택입니다. 1-4 사이의 숫자를 입력해주세요.");
        }
    }
}
