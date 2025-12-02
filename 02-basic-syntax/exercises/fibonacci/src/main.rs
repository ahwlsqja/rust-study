// ============================================================
// 실습: 피보나치 수열 계산기
// ============================================================
// n번째 피보나치 수를 계산하는 프로그램
// 학습 내용: 변수, 함수, 반복문, 재귀, 조건문
// ============================================================

use std::io;

// ============================================================
// 피보나치 수열이란?
// ┌─────────────────────────────────────────────────────────┐
// │  각 항이 앞의 두 항의 합인 수열                         │
// │                                                         │
// │  F(0) = 0                                               │
// │  F(1) = 1                                               │
// │  F(n) = F(n-1) + F(n-2)  (n >= 2)                       │
// │                                                         │
// │  수열: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...     │
// │        │  │  │  │  │                                    │
// │        │  │  │  │  └─ 2 + 3 = 5                         │
// │        │  │  │  └──── 1 + 2 = 3                         │
// │        │  │  └─────── 1 + 1 = 2                         │
// │        │  └────────── 0 + 1 = 1                         │
// │        └───────────── 초기값                            │
// │                                                         │
// │  자연에서 피보나치: 해바라기 씨앗, 솔방울, 조개 나선    │
// └─────────────────────────────────────────────────────────┘
// ============================================================

/// 반복문을 사용한 피보나치 계산 (권장)
///
/// # 장점
/// - 시간 복잡도: O(n)
/// - 공간 복잡도: O(1)
/// - 스택 오버플로우 걱정 없음
///
/// # 동작 원리
/// ┌─────────────────────────────────────────────────────────┐
/// │  n = 5 일 때 동작 과정                                  │
/// │                                                         │
/// │  초기:  a = 0, b = 1                                    │
/// │                                                         │
/// │  i=2:   temp = 0 + 1 = 1                                │
/// │         a = 1, b = 1                                    │
/// │                                                         │
/// │  i=3:   temp = 1 + 1 = 2                                │
/// │         a = 1, b = 2                                    │
/// │                                                         │
/// │  i=4:   temp = 1 + 2 = 3                                │
/// │         a = 2, b = 3                                    │
/// │                                                         │
/// │  i=5:   temp = 2 + 3 = 5                                │
/// │         a = 3, b = 5                                    │
/// │                                                         │
/// │  반환: b = 5                                            │
/// └─────────────────────────────────────────────────────────┘
fn fibonacci_iterative(n: u32) -> u64 {
    // 기저 사례 (base case)
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    // 두 개의 이전 값만 추적
    let mut a: u64 = 0;  // F(n-2)
    let mut b: u64 = 1;  // F(n-1)

    // 2부터 n까지 반복
    for _ in 2..=n {
        let temp = a + b;  // F(n) = F(n-1) + F(n-2)
        a = b;             // F(n-2) <- F(n-1)
        b = temp;          // F(n-1) <- F(n)
    }

    b  // F(n) 반환
}

/// 재귀를 사용한 피보나치 계산 (교육용, 비효율적)
///
/// # 단점
/// - 시간 복잡도: O(2^n) - 매우 느림!
/// - 중복 계산이 엄청나게 많음
/// - n이 크면 스택 오버플로우 위험
///
/// # 동작 원리 (재귀 트리)
/// ┌─────────────────────────────────────────────────────────┐
/// │  fib(5) 호출 시 재귀 트리                               │
/// │                                                         │
/// │                    fib(5)                               │
/// │                   /      \                              │
/// │              fib(4)      fib(3)                         │
/// │             /     \      /     \                        │
/// │         fib(3)  fib(2) fib(2) fib(1)                   │
/// │         /    \    |      |                              │
/// │     fib(2) fib(1) 1      1                              │
/// │       |                                                 │
/// │       1                                                 │
/// │                                                         │
/// │  같은 값이 여러 번 계산됨! (fib(2)가 3번, fib(3)이 2번) │
/// └─────────────────────────────────────────────────────────┘
fn fibonacci_recursive(n: u32) -> u64 {
    // 기저 사례
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    // 재귀 호출
    // 주의: 이 방식은 매우 비효율적임!
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

/// 피보나치 수열을 배열로 반환 (처음 n개)
fn fibonacci_sequence(count: usize) -> Vec<u64> {
    // Vec은 동적 배열 (07-collections에서 자세히)
    // vec![]은 빈 벡터를 생성하는 매크로
    let mut sequence = Vec::new();

    if count == 0 {
        return sequence;
    }

    sequence.push(0);
    if count == 1 {
        return sequence;
    }

    sequence.push(1);
    if count == 2 {
        return sequence;
    }

    for i in 2..count {
        // sequence[i-1] + sequence[i-2]
        let next = sequence[i - 1] + sequence[i - 2];
        sequence.push(next);
    }

    sequence
}

/// 숫자 입력 받기
fn read_number() -> Option<u32> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("입력 실패");

    input.trim().parse().ok()
}

/// 메뉴 출력
fn print_menu() {
    println!();
    println!("┌─────────────────────────────────┐");
    println!("│     피보나치 수열 계산기        │");
    println!("├─────────────────────────────────┤");
    println!("│  1. n번째 피보나치 수 (반복)    │");
    println!("│  2. n번째 피보나치 수 (재귀)    │");
    println!("│  3. 피보나치 수열 출력          │");
    println!("│  4. 두 방식 성능 비교           │");
    println!("│  5. 종료                        │");
    println!("└─────────────────────────────────┘");
    print!("선택: ");

    use std::io::Write;
    io::stdout().flush().unwrap();
}

fn main() {
    println!("====================================");
    println!("  피보나치 수열 계산기");
    println!("====================================");

    loop {
        print_menu();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("입력 실패");

        let choice = choice.trim();

        if choice == "1" {
            // 반복문 방식
            println!("\nn을 입력하세요 (0-93):");
            println!("(94 이상은 u64 오버플로우 발생)");

            if let Some(n) = read_number() {
                if n > 93 {
                    println!("너무 큰 수입니다. 93 이하로 입력해주세요.");
                } else {
                    let result = fibonacci_iterative(n);
                    println!();
                    println!("  F({}) = {}", n, result);
                }
            } else {
                println!("잘못된 입력입니다.");
            }

        } else if choice == "2" {
            // 재귀 방식 (느림 경고)
            println!("\nn을 입력하세요 (0-40 권장):");
            println!("(재귀는 느립니다. 40 이상은 오래 걸릴 수 있음)");

            if let Some(n) = read_number() {
                if n > 45 {
                    println!("너무 큰 수입니다. 재귀 방식은 45 이하만 지원합니다.");
                } else {
                    println!("계산 중...");
                    let result = fibonacci_recursive(n);
                    println!();
                    println!("  F({}) = {}", n, result);
                }
            } else {
                println!("잘못된 입력입니다.");
            }

        } else if choice == "3" {
            // 수열 출력
            println!("\n출력할 개수를 입력하세요 (1-50 권장):");

            if let Some(count) = read_number() {
                let count = count as usize;
                if count > 94 {
                    println!("94개까지만 출력 가능합니다.");
                } else if count == 0 {
                    println!("1 이상의 수를 입력해주세요.");
                } else {
                    let sequence = fibonacci_sequence(count);
                    println!();
                    println!("피보나치 수열 (처음 {}개):", count);
                    println!("─────────────────────────────────");
                    for (i, num) in sequence.iter().enumerate() {
                        print!("F({})={:<15} ", i, num);
                        // 3개마다 줄바꿈
                        if (i + 1) % 3 == 0 {
                            println!();
                        }
                    }
                    println!();
                }
            } else {
                println!("잘못된 입력입니다.");
            }

        } else if choice == "4" {
            // 성능 비교
            println!("\n비교할 n을 입력하세요 (1-40 권장):");

            if let Some(n) = read_number() {
                if n > 45 {
                    println!("재귀 방식은 45 이하만 비교 가능합니다.");
                } else {
                    println!();
                    println!("성능 비교 (n = {}):", n);
                    println!("─────────────────────────────────");

                    // 반복 방식 측정
                    use std::time::Instant;
                    let start = Instant::now();
                    let result_iter = fibonacci_iterative(n);
                    let duration_iter = start.elapsed();

                    // 재귀 방식 측정
                    let start = Instant::now();
                    let result_rec = fibonacci_recursive(n);
                    let duration_rec = start.elapsed();

                    println!("반복: F({}) = {}", n, result_iter);
                    println!("       소요 시간: {:?}", duration_iter);
                    println!();
                    println!("재귀: F({}) = {}", n, result_rec);
                    println!("       소요 시간: {:?}", duration_rec);
                    println!();

                    // 속도 비교
                    if duration_rec.as_nanos() > 0 && duration_iter.as_nanos() > 0 {
                        let ratio = duration_rec.as_nanos() as f64 / duration_iter.as_nanos() as f64;
                        if ratio > 1.0 {
                            println!("반복이 {:.1}배 빠릅니다!", ratio);
                        }
                    }
                }
            } else {
                println!("잘못된 입력입니다.");
            }

        } else if choice == "5" {
            println!("\n프로그램을 종료합니다.");
            break;

        } else {
            println!("잘못된 선택입니다. 1-5 사이의 숫자를 입력해주세요.");
        }
    }
}
