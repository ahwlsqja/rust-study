# 함수

## 개요

함수는 코드를 재사용 가능한 단위로 묶는 기본적인 구성 요소이다. Rust에서 함수는 `fn` 키워드로 정의하며, 매개변수 타입과 반환 타입을 명시해야 한다. 또한 Rust는 **표현식 기반 언어**로, 함수 본문의 마지막 표현식이 반환값이 된다.

---

## 이론

### 핵심 개념

#### 1. 함수 정의 기본

```rust
fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // 함수 본문
}
```

```
┌─────────────────────────────────────────────────────────────┐
│  함수 구조 분석                                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   fn add(x: i32, y: i32) -> i32 {                          │
│   ^^  ^^^  ^  ^^^   ^  ^^^    ^^^                          │
│   │   │    │   │    │   │      │                           │
│   │   │    │   │    │   │      └── 반환 타입               │
│   │   │    │   │    │   └── 매개변수 타입 (필수!)          │
│   │   │    │   │    └── 두 번째 매개변수                   │
│   │   │    │   └── 매개변수 타입 (필수!)                   │
│   │   │    └── 첫 번째 매개변수                            │
│   │   └── 함수 이름 (snake_case 규칙)                      │
│   └── 함수 정의 키워드                                     │
│                                                             │
│       x + y   <- 반환값 (세미콜론 없음!)                   │
│   }                                                         │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

#### 2. 명명 규칙

```
┌─────────────────────────────────────────────────────────────┐
│  Rust 명명 규칙                                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   종류              규칙              예시                  │
│   ─────────────────────────────────────────────────         │
│   함수/변수         snake_case        calculate_area        │
│   상수              SCREAMING_SNAKE   MAX_VALUE             │
│   타입/구조체       PascalCase        MyStruct              │
│   제네릭 타입       대문자 한 글자    T, E, K, V            │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

#### 3. 구문(Statement) vs 표현식(Expression)

Rust에서 매우 중요한 개념이다.

```
┌─────────────────────────────────────────────────────────────┐
│  구문 vs 표현식                                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   구문 (Statement)                                          │
│   ─────────────────                                         │
│   - 어떤 동작을 수행하고 값을 반환하지 않음                │
│   - 세미콜론(;)으로 끝남                                   │
│   - 예: let x = 5;                                         │
│                                                             │
│   표현식 (Expression)                                       │
│   ─────────────────                                         │
│   - 값으로 평가됨                                          │
│   - 세미콜론 없음                                          │
│   - 예: 5, x + 1, { let y = 3; y + 1 }                     │
│                                                             │
│   ┌────────────────────────────────────────────────────┐   │
│   │  let x = 5;         <- 구문 (값 반환 안 함)        │   │
│   │                                                    │   │
│   │  let y = {          <- 블록은 표현식!              │   │
│   │      let temp = 3;  <- 구문                        │   │
│   │      temp + 1       <- 표현식 (블록의 반환값)      │   │
│   │  };                 <- y = 4                       │   │
│   │                                                    │   │
│   │  let z = if true { 5 } else { 6 };  <- if도 표현식 │   │
│   └────────────────────────────────────────────────────┘   │
│                                                             │
│   주의: 표현식에 세미콜론을 붙이면 구문이 됨!              │
│         x + 1   <- 표현식 (값 반환)                        │
│         x + 1;  <- 구문 (값 반환 안 함, ()반환)            │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

#### 4. 반환값

```
┌─────────────────────────────────────────────────────────────┐
│  반환값 두 가지 방법                                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   방법 1: 마지막 표현식 (권장)                              │
│   ──────────────────────────                                │
│   fn add(a: i32, b: i32) -> i32 {                          │
│       a + b    <- 세미콜론 없음! 이 값이 반환됨            │
│   }                                                         │
│                                                             │
│   방법 2: return 키워드 (조기 반환 시 사용)                 │
│   ────────────────────────────────                          │
│   fn check_positive(x: i32) -> bool {                      │
│       if x < 0 {                                           │
│           return false;  <- 조기 반환                      │
│       }                                                     │
│       true              <- 마지막 표현식                   │
│   }                                                         │
│                                                             │
│   반환값 없는 함수:                                         │
│   ──────────────────                                        │
│   fn say_hello() {       <- 반환 타입 생략 = () 반환       │
│       println!("Hello!");                                   │
│   }                                                         │
│                                                             │
│   fn say_hello() -> () {  <- 명시적으로 () 작성도 가능     │
│       println!("Hello!");                                   │
│   }                                                         │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

#### 5. 함수 호출 순서

```
┌─────────────────────────────────────────────────────────────┐
│  함수 정의 위치                                             │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   Rust에서는 함수 정의 순서가 중요하지 않다!               │
│   (C와 다르게 전방 선언 불필요)                             │
│                                                             │
│   fn main() {                                               │
│       hello();      <- 아래에 정의된 함수 호출 가능        │
│   }                                                         │
│                                                             │
│   fn hello() {      <- main 뒤에 정의해도 OK               │
│       println!("Hello!");                                   │
│   }                                                         │
│                                                             │
│   컴파일러가 먼저 모든 함수 시그니처를 수집하기 때문       │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 문법/사용법

```rust
// 기본 함수
fn greet() {
    println!("안녕하세요!");
}

// 매개변수가 있는 함수
fn greet_person(name: &str) {
    println!("안녕하세요, {}님!", name);
}

// 반환값이 있는 함수
fn add(a: i32, b: i32) -> i32 {
    a + b  // 세미콜론 없음!
}

// 여러 값 반환 (튜플 사용)
fn get_min_max(numbers: &[i32]) -> (i32, i32) {
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();
    (min, max)
}

// 조기 반환
fn absolute(x: i32) -> i32 {
    if x < 0 {
        return -x;
    }
    x
}
```

### 주의사항

1. **매개변수 타입은 반드시 명시**
```rust
fn add(x, y) -> i32 {  // 에러! 타입 명시 필수
    x + y
}
```

2. **반환값에 세미콜론 주의**
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b;  // 에러! 세미콜론 때문에 ()가 반환됨
}
```

3. **함수 내 변수는 스코프에 제한됨**
```rust
fn outer() {
    let x = 5;
    inner();  // inner에서 x에 접근 불가
}

fn inner() {
    println!("{}", x);  // 에러! x는 outer의 지역 변수
}
```

---

## 실습

### 실습 1: 기본 함수 작성

목표: 간단한 함수를 정의하고 호출한다

코드:
```rust
// 반환값 없는 함수
fn say_hello() {
    println!("안녕하세요!");
}

// 매개변수가 있는 함수
fn say_hello_to(name: &str) {
    println!("안녕하세요, {}님!", name);
}

// 여러 매개변수
fn print_info(name: &str, age: i32) {
    println!("이름: {}, 나이: {}", name, age);
}

fn main() {
    say_hello();
    say_hello_to("홍길동");
    print_info("김철수", 25);
}
```

실행 결과:
```
안녕하세요!
안녕하세요, 홍길동님!
이름: 김철수, 나이: 25
```

### 실습 2: 반환값이 있는 함수

목표: 값을 반환하는 함수를 작성한다

코드:
```rust
// 두 수의 합
fn add(a: i32, b: i32) -> i32 {
    a + b  // 표현식 - 세미콜론 없음
}

// 두 수의 곱 (return 사용)
fn multiply(a: i32, b: i32) -> i32 {
    return a * b;  // return은 세미콜론 있어도 됨
}

// 제곱 계산
fn square(x: i32) -> i32 {
    x * x
}

// 원의 넓이
fn circle_area(radius: f64) -> f64 {
    const PI: f64 = 3.14159265359;
    PI * radius * radius
}

fn main() {
    let sum = add(5, 3);
    let product = multiply(4, 7);
    let squared = square(6);
    let area = circle_area(5.0);

    println!("5 + 3 = {}", sum);
    println!("4 * 7 = {}", product);
    println!("6^2 = {}", squared);
    println!("반지름 5인 원의 넓이: {:.2}", area);
}
```

실행 결과:
```
5 + 3 = 8
4 * 7 = 28
6^2 = 36
반지름 5인 원의 넓이: 78.54
```

### 실습 3: 표현식 활용

목표: Rust의 표현식 기반 특성을 이해한다

코드:
```rust
fn main() {
    // 블록도 표현식이다
    let y = {
        let x = 3;
        x + 1  // 세미콜론 없음 - 이 값이 블록의 결과
    };
    println!("블록 표현식: y = {}", y);

    // if도 표현식이다
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("if 표현식: number = {}", number);

    // 함수 호출도 표현식이다
    let result = add_one(5);
    println!("함수 호출 표현식: result = {}", result);

    // 중첩 표현식
    let complex = {
        let a = 10;
        let b = {
            let c = 2;
            c * 3  // 6
        };
        a + b  // 10 + 6 = 16
    };
    println!("중첩 표현식: complex = {}", complex);
}

fn add_one(x: i32) -> i32 {
    x + 1
}
```

실행 결과:
```
블록 표현식: y = 4
if 표현식: number = 5
함수 호출 표현식: result = 6
중첩 표현식: complex = 16
```

### 실습 4: 튜플로 여러 값 반환

목표: 함수에서 여러 값을 반환한다

코드:
```rust
// 두 수의 합과 차를 동시에 반환
fn sum_and_diff(a: i32, b: i32) -> (i32, i32) {
    (a + b, a - b)
}

// 나눗셈의 몫과 나머지 반환
fn div_mod(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}

// 배열의 최솟값, 최댓값, 합계 반환
fn array_stats(arr: &[i32]) -> (i32, i32, i32) {
    let mut min = arr[0];
    let mut max = arr[0];
    let mut sum = 0;

    for &num in arr {
        if num < min { min = num; }
        if num > max { max = num; }
        sum += num;
    }

    (min, max, sum)
}

fn main() {
    // 합과 차
    let (sum, diff) = sum_and_diff(10, 3);
    println!("10과 3의 합: {}, 차: {}", sum, diff);

    // 몫과 나머지
    let (quotient, remainder) = div_mod(17, 5);
    println!("17 / 5 = {} ... {}", quotient, remainder);

    // 배열 통계
    let numbers = [5, 2, 8, 1, 9, 3, 7];
    let (min, max, total) = array_stats(&numbers);
    println!("최솟값: {}, 최댓값: {}, 합계: {}", min, max, total);
}
```

실행 결과:
```
10과 3의 합: 13, 차: 7
17 / 5 = 3 ... 2
최솟값: 1, 최댓값: 9, 합계: 35
```

### 실습 5: 조기 반환 활용

목표: return 키워드로 조기 반환을 구현한다

코드:
```rust
// 양수인지 확인 (조기 반환)
fn is_positive(x: i32) -> bool {
    if x <= 0 {
        return false;  // 조기 반환
    }
    true  // 마지막 표현식
}

// 등급 계산 (여러 조건에서 조기 반환)
fn get_grade(score: i32) -> char {
    if score >= 90 {
        return 'A';
    }
    if score >= 80 {
        return 'B';
    }
    if score >= 70 {
        return 'C';
    }
    if score >= 60 {
        return 'D';
    }
    'F'
}

// 첫 번째 음수 찾기 (없으면 -1)
fn find_first_negative(arr: &[i32]) -> i32 {
    for &num in arr {
        if num < 0 {
            return num;  // 찾으면 즉시 반환
        }
    }
    -1  // 못 찾으면 -1 반환 (실제로는 Option 사용 권장)
}

fn main() {
    println!("5는 양수인가? {}", is_positive(5));
    println!("-3은 양수인가? {}", is_positive(-3));
    println!("0은 양수인가? {}", is_positive(0));

    println!("\n점수 85의 등급: {}", get_grade(85));
    println!("점수 92의 등급: {}", get_grade(92));
    println!("점수 55의 등급: {}", get_grade(55));

    let nums1 = [1, 2, -3, 4, -5];
    let nums2 = [1, 2, 3, 4, 5];
    println!("\n첫 번째 음수: {}", find_first_negative(&nums1));
    println!("첫 번째 음수: {}", find_first_negative(&nums2));
}
```

실행 결과:
```
5는 양수인가? true
-3은 양수인가? false
0은 양수인가? false

점수 85의 등급: B
점수 92의 등급: A
점수 55의 등급: F

첫 번째 음수: -3
첫 번째 음수: -1
```

---

## 정리

### 오늘 배운 것
- 함수는 `fn` 키워드로 정의하며, 이름은 snake_case 규칙을 따른다
- 매개변수 타입과 반환 타입은 반드시 명시해야 한다
- Rust는 표현식 기반 언어로, 마지막 표현식이 반환값이 된다
- 세미콜론의 유무가 구문/표현식을 결정한다 (매우 중요!)
- `return` 키워드는 조기 반환에 사용한다
- 튜플을 사용하여 여러 값을 반환할 수 있다

### 질문/의문점
- 함수 포인터나 함수를 변수에 저장할 수 있는가?
- 클로저(익명 함수)는 어떻게 사용하는가?
- 제네릭 함수는 어떻게 작성하는가?

### 추가 학습 필요
- 클로저 (11-closures-iterators에서 학습)
- 제네릭 함수 (09-generics-traits-lifetimes에서 학습)
- 메서드 (04-structs에서 학습)

---

## 참고 자료
- The Rust Programming Language - Functions: https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
- Rust by Example - Functions: https://doc.rust-lang.org/rust-by-example/fn.html
