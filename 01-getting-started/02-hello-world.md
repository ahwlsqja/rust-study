# Hello World

## 개요

모든 프로그래밍 언어 학습의 시작점인 Hello World 프로그램을 Rust로 작성한다. 이를 통해 Rust 파일 구조, 컴파일 과정, 기본 문법을 이해한다.

---

## 이론

### 핵심 개념

Rust 프로그램의 기본 구조:

```rust
fn main() {
    println!("Hello, World!");
}
```

- `fn`: 함수를 정의하는 키워드
- `main`: 프로그램의 진입점(entry point). 모든 실행 가능한 Rust 프로그램은 main 함수가 필요하다
- `println!`: 표준 출력에 텍스트를 출력하는 매크로 (함수가 아님, `!`가 매크로임을 나타냄)
- `{}`: 코드 블록을 나타내는 중괄호
- `;`: 문(statement)의 끝을 나타내는 세미콜론

### 문법/사용법

**파일 확장자**: `.rs`

**컴파일 방법**:
```bash
rustc main.rs
```

**실행 방법**:
```bash
# Windows
.\main.exe

# Linux/macOS
./main
```

### 동작 원리

1. `rustc` 컴파일러가 소스 코드를 읽는다
2. 구문 분석(parsing) 및 타입 검사를 수행한다
3. 중간 표현(IR)으로 변환한다
4. LLVM 백엔드를 통해 기계어로 컴파일한다
5. 실행 가능한 바이너리 파일을 생성한다

Rust는 **AOT(Ahead-Of-Time) 컴파일** 언어로, 실행 전에 미리 기계어로 컴파일된다.

### 매크로 vs 함수

`println!`에서 `!`는 이것이 함수가 아니라 **매크로**임을 나타낸다.

| 구분 | 함수 | 매크로 |
|------|------|--------|
| 호출 | `function()` | `macro!()` |
| 인자 개수 | 고정 | 가변 |
| 컴파일 시점 | 런타임에 호출 | 컴파일 타임에 코드 확장 |

### 주의사항

- 세미콜론(`;`)을 빠뜨리면 컴파일 에러 발생
- `main` 함수는 반드시 있어야 한다
- Rust는 들여쓰기로 4칸 스페이스를 권장한다 (탭 대신)
- `println!`에서 `!`를 빠뜨리면 함수를 찾을 수 없다는 에러 발생

---

## 실습

### 실습 1: 기본 Hello World

목표: rustc를 사용하여 직접 컴파일하고 실행한다

코드:
```rust
// main.rs
fn main() {
    println!("Hello, World!");
}
```

실행 방법:
```bash
# 임시 디렉토리 생성
mkdir hello_world
cd hello_world

# 파일 생성 후 컴파일
rustc main.rs

# 실행
.\main.exe
```

실행 결과:
```
Hello, World!
```

설명:
- `rustc`는 단일 파일을 직접 컴파일하는 저수준 방법이다
- 실제 프로젝트에서는 cargo를 사용한다 (다음 장에서 학습)

### 실습 2: 다양한 출력

목표: println! 매크로의 다양한 사용법을 익힌다

코드:
```rust
fn main() {
    // 기본 출력
    println!("Hello, Rust!");

    // 변수 삽입 (포맷팅)
    let name = "Rustacean";
    println!("Hello, {}!", name);

    // 여러 값 출력
    let x = 5;
    let y = 10;
    println!("x = {}, y = {}", x, y);

    // 위치 지정
    println!("{0}은 {1}보다 작고, {1}은 {0}보다 크다", x, y);

    // 이름 지정
    println!("{name}님, 환영합니다!", name = "홍길동");

    // 디버그 출력
    println!("{:?}", (1, 2, 3));
}
```

실행 결과:
```
Hello, Rust!
Hello, Rustacean!
x = 5, y = 10
5은 10보다 작고, 10은 5보다 크다
홍길동님, 환영합니다!
(1, 2, 3)
```

설명:
- `{}`: 기본 포맷 플레이스홀더
- `{0}`, `{1}`: 위치 기반 인자 지정
- `{name}`: 이름 기반 인자 지정
- `{:?}`: 디버그 포맷 (Debug 트레이트 사용)

### 실습 3: 여러 줄 출력과 이스케이프

목표: 특수 문자와 여러 줄 문자열 처리를 익힌다

코드:
```rust
fn main() {
    // 이스케이프 시퀀스
    println!("탭:\t다음 내용");
    println!("줄바꿈:\n새로운 줄");
    println!("따옴표: \"인용문\"");
    println!("백슬래시: \\");

    // 여러 줄 문자열
    println!("첫째 줄
둘째 줄
셋째 줄");

    // Raw 문자열 (이스케이프 무시)
    println!(r"C:\Users\user\Documents");
    println!(r#"JSON: {"key": "value"}"#);
}
```

실행 결과:
```
탭:	다음 내용
줄바꿈:
새로운 줄
따옴표: "인용문"
백슬래시: \
첫째 줄
둘째 줄
셋째 줄
C:\Users\user\Documents
JSON: {"key": "value"}
```

설명:
- `\t`: 탭 문자
- `\n`: 줄바꿈
- `\"`: 큰따옴표
- `\\`: 백슬래시
- `r"..."`: Raw 문자열 (이스케이프 처리 안 함)
- `r#"..."#`: 큰따옴표를 포함할 수 있는 Raw 문자열

---

## 정리

### 오늘 배운 것
- Rust 프로그램은 `main` 함수에서 시작한다
- `println!`은 함수가 아니라 매크로이다 (`!`로 구분)
- `rustc`로 직접 컴파일할 수 있다
- `{}`를 사용해 문자열 포맷팅이 가능하다
- Raw 문자열(`r"..."`)로 이스케이프 없이 문자열을 작성할 수 있다

### 질문/의문점
- 매크로와 함수의 차이점은 정확히 무엇인가?
- `println!`은 어떻게 가변 인자를 받을 수 있는가?
- 컴파일된 바이너리의 크기가 큰 이유는?

### 추가 학습 필요
- 매크로 시스템 심화
- 다른 출력 매크로 (`print!`, `eprintln!`, `format!`)
- 포맷 지정자 심화 (`{:05}`, `{:.2}` 등)

---

## 참고 자료
- The Rust Programming Language - Hello, World!: https://doc.rust-lang.org/book/ch01-02-hello-world.html
- std::fmt 문서: https://doc.rust-lang.org/std/fmt/
- Rust by Example - Hello World: https://doc.rust-lang.org/rust-by-example/hello.html
