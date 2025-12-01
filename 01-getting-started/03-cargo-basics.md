# Cargo 기초

## 개요

Cargo는 Rust의 공식 빌드 시스템이자 패키지 매니저이다. 프로젝트 생성, 의존성 관리, 빌드, 테스트, 문서 생성 등 개발에 필요한 거의 모든 작업을 수행한다. 실제 Rust 개발에서는 rustc 대신 cargo를 사용한다.

---

## 이론

### 핵심 개념

**Cargo의 역할**:
1. 프로젝트 구조 생성 및 관리
2. 의존성(dependencies) 관리
3. 코드 컴파일
4. 배포 가능한 패키지 생성
5. crates.io에 패키지 업로드

**용어 정리**:
- **Crate**: Rust의 컴파일 단위. 라이브러리 또는 실행 파일
- **Package**: 하나 이상의 crate를 포함하는 Cargo 프로젝트
- **crates.io**: Rust 커뮤니티의 패키지 저장소

### 주요 명령어

| 명령어 | 설명 |
|--------|------|
| `cargo new <name>` | 새 프로젝트 생성 (바이너리) |
| `cargo new <name> --lib` | 새 라이브러리 프로젝트 생성 |
| `cargo init` | 현재 디렉토리를 프로젝트로 초기화 |
| `cargo build` | 프로젝트 빌드 (debug 모드) |
| `cargo build --release` | 최적화된 릴리스 빌드 |
| `cargo run` | 빌드 후 실행 |
| `cargo check` | 컴파일 가능 여부만 확인 (빠름) |
| `cargo test` | 테스트 실행 |
| `cargo doc` | 문서 생성 |
| `cargo clean` | 빌드 결과물 삭제 |
| `cargo update` | 의존성 업데이트 |

### 프로젝트 구조

```
my_project/
├── Cargo.toml      # 프로젝트 설정 파일
├── Cargo.lock      # 의존성 버전 잠금 파일 (자동 생성)
├── src/
│   ├── main.rs     # 바이너리 크레이트 진입점
│   └── lib.rs      # 라이브러리 크레이트 진입점 (선택)
├── tests/          # 통합 테스트
├── benches/        # 벤치마크
├── examples/       # 예제 코드
└── target/         # 빌드 결과물 (자동 생성)
```

### Cargo.toml

프로젝트의 메타데이터와 의존성을 정의하는 파일:

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "A short description"

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }

[dev-dependencies]
# 개발/테스트 시에만 필요한 의존성

[build-dependencies]
# 빌드 스크립트용 의존성
```

### Debug vs Release 빌드

| 구분 | Debug | Release |
|------|-------|---------|
| 명령어 | `cargo build` | `cargo build --release` |
| 최적화 | 없음 | 최대 |
| 컴파일 속도 | 빠름 | 느림 |
| 실행 속도 | 느림 | 빠름 |
| 디버그 정보 | 포함 | 제외 |
| 출력 위치 | `target/debug/` | `target/release/` |

### 주의사항

- `Cargo.lock`은 바이너리 프로젝트에서는 버전 관리에 포함, 라이브러리에서는 제외하는 것이 일반적
- `target/` 폴더는 .gitignore에 추가해야 한다
- `cargo check`는 `cargo build`보다 빠르므로 개발 중 자주 사용한다
- edition은 Rust의 에디션 (2015, 2018, 2021)을 지정한다

---

## 실습

### 실습 1: 새 프로젝트 생성

목표: cargo를 사용하여 새 프로젝트를 생성하고 구조를 확인한다

코드:
```bash
# 새 프로젝트 생성
cargo new hello_cargo

# 프로젝트 디렉토리로 이동
cd hello_cargo

# 구조 확인
tree .
# 또는
dir /s /b
```

실행 결과:
```
     Created binary (application) `hello_cargo` package

hello_cargo/
├── Cargo.toml
└── src/
    └── main.rs
```

생성된 main.rs:
```rust
fn main() {
    println!("Hello, world!");
}
```

생성된 Cargo.toml:
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

설명:
- `cargo new`는 자동으로 git 저장소도 초기화한다 (`--vcs none`으로 비활성화 가능)
- 기본 main.rs에는 Hello World 코드가 포함되어 있다

### 실습 2: 빌드와 실행

목표: cargo build와 cargo run의 차이를 이해한다

코드:
```bash
# 빌드만 수행
cargo build

# 빌드된 파일 확인
dir target\debug\

# 직접 실행
.\target\debug\hello_cargo.exe

# 빌드 + 실행 (한 번에)
cargo run

# 릴리스 빌드
cargo build --release
dir target\release\
```

실행 결과:
```
   Compiling hello_cargo v0.1.0 (C:\Users\user\hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s

Hello, world!

   Compiling hello_cargo v0.1.0 (C:\Users\user\hello_cargo)
    Finished release [optimized] target(s) in 0.30s
```

설명:
- `cargo build`: 빌드만 수행
- `cargo run`: 빌드 후 자동 실행 (변경 사항 없으면 재빌드 안 함)
- `--release`: 최적화된 바이너리 생성 (배포용)

### 실습 3: cargo check 활용

목표: 빠른 오류 확인을 위해 cargo check를 사용한다

코드:
```rust
// src/main.rs - 의도적 오류 포함
fn main() {
    let x: i32 = "hello";  // 타입 오류
    println!("{}", x);
}
```

```bash
# 빠른 검사
cargo check

# build와 비교
cargo build
```

실행 결과:
```
error[E0308]: mismatched types
 --> src/main.rs:2:18
  |
2 |     let x: i32 = "hello";
  |            ---   ^^^^^^^ expected `i32`, found `&str`
  |            |
  |            expected due to this

error: aborting due to previous error
```

설명:
- `cargo check`는 바이너리 생성 없이 코드 검사만 수행
- `cargo build`보다 훨씬 빠르다 (대규모 프로젝트에서 유용)
- 코드 작성 중 자주 실행하여 오류를 빠르게 확인할 수 있다

### 실습 4: 의존성 추가

목표: 외부 크레이트를 추가하고 사용한다

Cargo.toml 수정:
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"
```

src/main.rs:
```rust
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(1..=100);
    println!("Random number: {}", n);
}
```

```bash
cargo build
cargo run
```

실행 결과:
```
   Compiling libc v0.2.150
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.17
   Compiling getrandom v0.2.11
   Compiling rand_core v0.6.4
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling hello_cargo v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 2.50s
     Running `target\debug\hello_cargo.exe`
Random number: 42
```

설명:
- 의존성을 추가하면 cargo가 자동으로 다운로드하고 컴파일한다
- 의존성의 의존성(transitive dependencies)도 자동 처리된다
- `Cargo.lock` 파일에 정확한 버전이 기록된다

---

## 정리

### 오늘 배운 것
- cargo는 Rust의 빌드 시스템이자 패키지 매니저이다
- `cargo new`로 프로젝트 생성, `cargo build`로 빌드, `cargo run`으로 실행
- `cargo check`는 빠른 오류 확인에 유용하다
- Cargo.toml에 의존성을 추가하면 자동으로 다운로드된다
- Debug와 Release 빌드의 차이를 알아야 한다

### 질문/의문점
- Cargo.lock을 버전 관리에 포함해야 하는 경우는?
- 의존성 버전 지정 방법 (`^`, `~`, `*` 등)의 차이는?
- workspace란 무엇인가?

### 추가 학습 필요
- Cargo.toml 상세 설정 (features, profiles 등)
- 커스텀 빌드 스크립트 (build.rs)
- cargo 확장 명령어 (cargo-watch, cargo-edit 등)

---

## 참고 자료
- The Rust Programming Language - Hello, Cargo!: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
- The Cargo Book: https://doc.rust-lang.org/cargo/
- crates.io: https://crates.io/
