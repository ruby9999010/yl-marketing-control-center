# Y&L Marketing - 중앙 관제 시스템

여러 대의 Y&L Marketing 클라이언트를 중앙에서 모니터링하는 관제 시스템입니다.

## 🚀 빌드 방법

### 방법 1: GitHub Actions (추천 - 로컬 환경 불필요)
[GITHUB_BUILD_GUIDE.md](./GITHUB_BUILD_GUIDE.md) 참고

### 방법 2: 로컬 빌드
[BUILD_GUIDE.md](./BUILD_GUIDE.md) 참고

## 기능

### 실시간 모니터링
- 클라이언트 연결 상태 확인
- 작업 성공/실패 카운트
- 다음 실행까지 남은 시간
- 현재 동작 상태

### 로그 관리
- 실시간 로그 수신 및 표시
- 로그 레벨별 필터링 (info/success/error/warning)
- 로그 검색
- 클라이언트별 로그 필터링

### 메모 기능
- 클라이언트별 독립 메모
- 자동 저장 (3초 후)
- 서버 용도, 담당자, 특이사항 등 기록

## 시작하기

### 개발 모드

```bash
npm install
npm run tauri dev
```

### 빌드

```bash
npm run tauri build
```

## 사용 방법

### 1. 관제 서버 실행
- 관제 서버를 실행하면 자동으로 WebSocket 서버가 시작됩니다 (기본 포트: 9999)

### 2. 클라이언트 연결
- 각 클라이언트에서 설정 → 관제 센터 탭으로 이동
- 식별 코드 입력 (예: ABC-001)
- 관제 서버 URL 입력 (예: ws://192.168.1.100:9999)
- 연결 활성화

### 3. 모니터링
- 좌측: 연결된 클라이언트 목록 및 상태
- 중앙: 실시간 로그 스트림
- 우측: 선택한 클라이언트의 메모

## 아키텍처

```
관제 서버 (포트 9999)
    ↓ WebSocket
클라이언트 #1 (ABC-001)
클라이언트 #2 (ABC-002)
클라이언트 #N (XYZ-999)
```

## 데이터베이스

SQLite 데이터베이스 위치:
- Windows: `%APPDATA%\com.svelion\yl-marketing-control-center\control_center.db`
- macOS: `~/Library/Application Support/com.svelion.yl-marketing-control-center/control_center.db`
- Linux: `~/.local/share/com.svelion.yl-marketing-control-center/control_center.db`

## 기술 스택

- **프론트엔드**: React 19 + TypeScript + TailwindCSS
- **백엔드**: Rust + Tauri 2
- **통신**: WebSocket (tokio-tungstenite)
- **데이터베이스**: SQLite (sqlx)

## 라이센스

Proprietary
