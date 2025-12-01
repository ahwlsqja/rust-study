# Rust 설치 및 환경 설정

## 개요

Rust를 설치하고 개발 환경을 구성하는 방법을 학습한다. Rust는 rustup이라는 도구를 통해 설치 및 버전 관리를 수행한다.

---

## 이론

### 핵심 개념

**rustup**은 Rust의 공식 설치 및 버전 관리 도구이다. 다음 기능을 제공한다:
- Rust 컴파일러(rustc) 설치
- 패키지 매니저(cargo) 설치
- 여러 버전의 Rust 관리 (stable, beta, nightly)
- 크로스 컴파일을 위한 타겟 추가

### 설치 방법

#### Windows

1. [rustup-init.exe](https://rustup.rs/) 다운로드 및 실행
2. 또는 PowerShell에서:
```powershell
winget install Rustlang.Rustup
```

#### Linux/macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 설치 확인

```bash
rustc --version
cargo --version
rustup --version
```

### 주요 rustup 명령어

| 명령어 | 설명 |
|--------|------|
| `rustup update` | Rust를 최신 버전으로 업데이트 |
| `rustup default stable` | stable 버전을 기본으로 설정 |
| `rustup show` | 설치된 툴체인 확인 |
| `rustup doc` | 로컬 문서 열기 |
| `rustup component add rustfmt` | 컴포넌트 추가 |

### 개발 환경 설정 (VS Code)

1. VS Code 설치
2. rust-analyzer 확장 설치 (공식 권장)
3. 추천 추가 확장:
   - CodeLLDB (디버깅)
   - Even Better TOML (Cargo.toml 지원)
   - Error Lens (인라인 에러 표시)

### 주의사항

- Windows에서는 Visual Studio Build Tools가 필요할 수 있다 (C++ 빌드 도구)
- PATH 환경 변수가 자동으로 설정되지 않으면 수동으로 `~/.cargo/bin`을 추가해야 한다
- 회사/학교 네트워크에서는 프록시 설정이 필요할 수 있다

---

## 실습

### 실습 1: 설치 상태 확인

목표: 현재 시스템의 Rust 설치 상태를 확인한다

코드:
```bash
rustc --version
cargo --version
rustup show
```

실행 결과:
```
rustc 1.83.0 (90b35a623 2024-11-26)
cargo 1.83.0 (5ffbef321 2024-10-29)

Default host: x86_64-pc-windows-msvc
rustup home:  C:\Users\user\.rustup

stable-x86_64-pc-windows-msvc (default)
rustc 1.83.0 (90b35a623 2024-11-26)
```

설명:
- `rustc --version`: Rust 컴파일러 버전 확인
- `cargo --version`: 패키지 매니저 버전 확인
- `rustup show`: 설치된 툴체인과 기본 설정 확인

### 실습 2: 로컬 문서 열기

목표: 오프라인에서도 사용 가능한 Rust 공식 문서를 확인한다

코드:
```bash
rustup doc
rustup doc --book
rustup doc --std
```

실행 결과:
```
(브라우저에서 Rust 문서가 열림)
```

설명:
- `rustup doc`: Rust 문서 메인 페이지
- `rustup doc --book`: The Rust Programming Language 책
- `rustup doc --std`: 표준 라이브러리 문서

---

## 정리

### 오늘 배운 것
- rustup은 Rust의 공식 설치 및 버전 관리 도구이다
- rustc는 컴파일러, cargo는 패키지 매니저이다
- VS Code + rust-analyzer가 권장 개발 환경이다
- `rustup doc`으로 오프라인 문서를 볼 수 있다

### 질문/의문점
- nightly 버전은 언제 사용하는가?
- 여러 프로젝트에서 다른 Rust 버전을 사용해야 할 때는 어떻게 하는가?

### 추가 학습 필요
- rustup의 고급 기능 (툴체인 오버라이드, 컴포넌트 관리)
- 크로스 컴파일 설정

---

## 참고 자료
- Rust 공식 설치 가이드: https://www.rust-lang.org/tools/install
- rustup 문서: https://rust-lang.github.io/rustup/
- VS Code Rust 설정: https://code.visualstudio.com/docs/languages/rust
