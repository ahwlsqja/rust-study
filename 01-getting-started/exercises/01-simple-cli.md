# 실습: 간단한 CLI 프로그램 만들기

## 과제 설명

사용자로부터 이름을 입력받아 인사 메시지를 출력하는 간단한 CLI 프로그램을 작성한다.

## 요구사항

1. 프로그램 실행 시 "이름을 입력하세요: " 메시지 출력
2. 사용자 입력을 받음
3. "안녕하세요, [이름]님!" 형식으로 인사 메시지 출력
4. cargo 프로젝트로 구성

## 힌트

- 표준 입력을 받으려면 `std::io` 모듈을 사용한다
- `String::new()`로 빈 문자열 생성
- `io::stdin().read_line(&mut buffer)`로 입력 읽기
- `.trim()`으로 줄바꿈 문자 제거

## 정답 코드

```rust
use std::io;

fn main() {
    println!("이름을 입력하세요: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("입력을 읽는 데 실패했습니다");

    let name = name.trim();

    println!("안녕하세요, {}님!", name);
}
```

## 실행 예시

```
이름을 입력하세요:
홍길동
안녕하세요, 홍길동님!
```

## 코드 설명

1. `use std::io;` - 표준 입출력 모듈을 가져온다
2. `String::new()` - 힙에 할당되는 가변 문자열 생성
3. `let mut name` - 변경 가능한 변수 선언 (입력을 받아야 하므로)
4. `read_line(&mut name)` - 표준 입력에서 한 줄을 읽어 name에 저장
   - `&mut`은 가변 참조를 의미한다 (나중에 자세히 학습)
5. `.expect()` - Result 타입의 에러 처리 (나중에 자세히 학습)
6. `.trim()` - 문자열 앞뒤의 공백과 줄바꿈 제거

## 추가 도전 과제

### 도전 1: 여러 정보 입력받기

이름, 나이, 직업을 입력받아 자기소개 문장을 출력하라.

```
이름: 홍길동
나이: 25
직업: 개발자

안녕하세요! 저는 25살 개발자 홍길동입니다.
```

### 도전 2: 반복 입력

"quit"을 입력할 때까지 계속 이름을 입력받아 인사하는 프로그램을 작성하라.

```
이름을 입력하세요 (종료: quit):
홍길동
안녕하세요, 홍길동님!
이름을 입력하세요 (종료: quit):
김철수
안녕하세요, 김철수님!
이름을 입력하세요 (종료: quit):
quit
프로그램을 종료합니다.
```

힌트: `loop`와 `break`를 사용한다

## 도전 과제 정답

### 도전 1 정답

```rust
use std::io;

fn main() {
    let mut name = String::new();
    let mut age = String::new();
    let mut job = String::new();

    println!("이름: ");
    io::stdin().read_line(&mut name).expect("입력 실패");

    println!("나이: ");
    io::stdin().read_line(&mut age).expect("입력 실패");

    println!("직업: ");
    io::stdin().read_line(&mut job).expect("입력 실패");

    println!(
        "\n안녕하세요! 저는 {}살 {} {}입니다.",
        age.trim(),
        job.trim(),
        name.trim()
    );
}
```

### 도전 2 정답

```rust
use std::io;

fn main() {
    loop {
        println!("이름을 입력하세요 (종료: quit):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("입력 실패");

        let input = input.trim();

        if input == "quit" {
            println!("프로그램을 종료합니다.");
            break;
        }

        println!("안녕하세요, {}님!", input);
    }
}
```

## 학습 포인트

이 실습을 통해 다음 개념을 미리 접했다:
- `use` 키워드로 모듈 가져오기
- `mut` 키워드로 가변 변수 선언
- `&mut` 가변 참조 (소유권 챕터에서 자세히 학습)
- `Result` 타입과 `.expect()` (에러 처리 챕터에서 자세히 학습)
- `loop`와 `break` (제어 흐름)
- 문자열 메서드 (`.trim()`)

이 개념들은 앞으로 더 자세히 배우게 된다. 지금은 패턴만 익혀두면 된다.
