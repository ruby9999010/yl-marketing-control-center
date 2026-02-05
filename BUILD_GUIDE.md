# 🏗️ Windows EXE 빌드 가이드

## 📋 사전 요구사항

### 1. Rust 설치
1. https://rustup.rs/ 접속
2. `rustup-init.exe` 다운로드 및 실행
3. 기본 설정으로 설치 (1번 선택)
4. 설치 완료 후 터미널 재시작

확인:
```powershell
rustc --version
cargo --version
```

### 2. Node.js 설치
1. https://nodejs.org/ 접속
2. LTS 버전 다운로드 (현재 20.x)
3. 설치 (기본 설정)

확인:
```powershell
node --version
npm --version
```

### 3. Visual Studio Build Tools (선택사항)
Rust가 자동으로 설치하지만, 문제가 있다면:
1. https://visualstudio.microsoft.com/downloads/
2. "Build Tools for Visual Studio" 다운로드
3. "Desktop development with C++" 워크로드 선택

---

## 🚀 빌드 방법

### 1단계: 의존성 설치

```powershell
cd control-center
npm install
```

### 2단계: 개발 모드 테스트 (선택사항)

```powershell
npm run tauri dev
```

데스크톱 앱이 실행됩니다. 정상 작동 확인 후 종료.

### 3단계: 프로덕션 빌드

```powershell
npm run tauri build
```

빌드 시간: 약 5-10분 (첫 빌드는 더 오래 걸림)

---

## 📦 빌드 결과물

빌드 완료 후 생성되는 파일:

```
control-center/src-tauri/target/release/
│
├── yl-marketing-control-center.exe          # 단독 실행 파일 (~5MB)
│   └─ 이 파일만 복사해서 실행 가능
│
└── bundle/
    └── nsis/
        └── Y.L.Marketing Control Center_1.0.0_x64-setup.exe  # 인스톨러 (~7MB)
            └─ 설치 프로그램 (권장)
```

### 배포 방법 선택

#### 방법 1: 단독 실행 파일 (간단)
- `yl-marketing-control-center.exe` 파일만 복사
- 어디서든 실행 가능
- 설치 불필요

#### 방법 2: 인스톨러 (권장)
- `Y.L.Marketing Control Center_1.0.0_x64-setup.exe` 실행
- 자동으로 설치
- 시작 메뉴에 바로가기 생성
- 제거 프로그램에 등록

---

## 🎯 사용 방법

### 개발자 (관제 서버)

1. **실행**
   ```powershell
   # 빌드된 EXE 실행
   .\src-tauri\target\release\yl-marketing-control-center.exe
   ```

2. **확인**
   - 앱이 실행되면 자동으로 WebSocket 서버 시작 (포트 9999)
   - 상단에 "WebSocket 서버 실행 중" 표시

3. **방화벽 설정**
   - Windows 방화벽 알림이 뜨면 "액세스 허용" 클릭
   - 또는 수동으로 포트 9999 열기:
   ```powershell
   netsh advfirewall firewall add rule name="YL Control Center" dir=in action=allow protocol=TCP localport=9999
   ```

4. **IP 주소 확인**
   ```powershell
   ipconfig
   ```
   IPv4 주소를 클라이언트에게 알려주기 (예: 192.168.1.100)

### 클라이언트 (사용자)

1. Y&L Marketing 프로그램 실행
2. 설정 → 관제 센터 탭
3. 식별 코드 입력 (예: ABC-001)
4. 서버 URL 입력: `ws://개발자IP:9999` (예: ws://192.168.1.100:9999)
5. 연결 활성화 토글 ON
6. 저장

---

## 🔧 빌드 옵션

### 릴리즈 최적화 (기본)
```powershell
npm run tauri build
```

### 디버그 빌드 (문제 해결용)
```powershell
npm run tauri build -- --debug
```

### 특정 타겟만 빌드
```toml
# src-tauri/tauri.conf.json 수정
"bundle": {
  "targets": ["nsis"]  # 또는 "msi", "app", "dmg" 등
}
```

---

## 🐛 문제 해결

### 빌드 실패: "rustc not found"
```powershell
# Rust 재설치
rustup update
```

### 빌드 실패: "link.exe not found"
Visual Studio Build Tools 설치 필요

### 실행 실패: "WebView2 not found"
Windows 11은 기본 설치됨. Windows 10이면:
https://developer.microsoft.com/microsoft-edge/webview2/

### 방화벽 차단
```powershell
# 방화벽 규칙 추가
netsh advfirewall firewall add rule name="YL Control Center" dir=in action=allow protocol=TCP localport=9999
```

### 포트 9999 이미 사용 중
```powershell
# 포트 사용 확인
netstat -ano | findstr :9999

# 프로세스 종료
taskkill /PID <PID번호> /F
```

---

## 📊 빌드 크기 최적화

현재 설정으로 이미 최적화됨:
- **EXE 크기**: ~5MB
- **인스톨러**: ~7MB
- **메모리 사용**: ~50MB

추가 최적화 (선택사항):
```toml
# src-tauri/Cargo.toml
[profile.release]
opt-level = "z"        # 크기 최적화 (현재 "s")
lto = "fat"           # 전체 LTO (현재 "thin")
codegen-units = 1     # 단일 코드 유닛 (현재 16)
strip = true          # 디버그 심볼 제거 (이미 설정됨)
```

---

## 🔒 보안

### 코드 서명 (선택사항)
Windows Defender SmartScreen 경고를 피하려면:

1. 코드 서명 인증서 구매
2. `tauri.conf.json`에 설정:
```json
"bundle": {
  "windows": {
    "certificateThumbprint": "YOUR_CERT_THUMBPRINT",
    "digestAlgorithm": "sha256",
    "timestampUrl": "http://timestamp.digicert.com"
  }
}
```

### 바이러스 백신 오탐지
- 첫 빌드는 오탐지 가능
- VirusTotal에 업로드하여 확인
- 필요시 백신 업체에 오탐 신고

---

## 📝 버전 관리

버전 업데이트:
```json
// control-center/package.json
{
  "version": "1.0.1"  // 수정
}

// control-center/src-tauri/Cargo.toml
[package]
version = "1.0.1"  // 수정

// control-center/src-tauri/tauri.conf.json
{
  "version": "1.0.1"  // 수정
}
```

---

## 🚀 자동 빌드 (CI/CD)

GitHub Actions 예시:
```yaml
name: Build
on: [push]
jobs:
  build:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: npm install
      - run: npm run tauri build
      - uses: actions/upload-artifact@v3
        with:
          name: windows-exe
          path: src-tauri/target/release/bundle/nsis/*.exe
```

---

## 📞 지원

문제가 발생하면:
1. 빌드 로그 확인
2. `npm run tauri build -- --verbose` 실행
3. 에러 메시지 복사
4. 이슈 등록

---

**🎉 빌드 완료 후 `yl-marketing-control-center.exe`를 실행하면 데스크톱 앱이 시작됩니다!**
