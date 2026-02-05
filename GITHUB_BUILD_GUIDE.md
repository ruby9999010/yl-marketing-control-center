# 🚀 GitHub Actions 자동 빌드 가이드

로컬 환경 설정 없이 GitHub에서 자동으로 Windows EXE를 빌드하는 방법입니다.

## 📋 준비물

- GitHub 계정
- Git 설치 (https://git-scm.com/)

---

## 🎯 방법 1: 새 저장소 생성 (추천)

### 1단계: GitHub 저장소 생성

1. https://github.com/new 접속
2. Repository name: `yl-marketing-control-center`
3. Private 선택 (비공개)
4. Create repository 클릭

### 2단계: 코드 업로드

```powershell
# control-center 폴더로 이동
cd control-center

# Git 초기화
git init

# 모든 파일 추가
git add .

# 커밋
git commit -m "Initial commit"

# GitHub 저장소 연결 (YOUR_USERNAME을 본인 계정으로 변경)
git remote add origin https://github.com/YOUR_USERNAME/yl-marketing-control-center.git

# 업로드
git branch -M main
git push -u origin main
```

### 3단계: 자동 빌드 확인

1. GitHub 저장소 페이지로 이동
2. 상단 "Actions" 탭 클릭
3. "Build Windows EXE" 워크플로우 확인
4. 빌드 완료까지 약 10-15분 대기

### 4단계: 빌드 파일 다운로드

1. Actions 탭에서 완료된 워크플로우 클릭
2. 하단 "Artifacts" 섹션에서 다운로드:
   - `windows-exe` - 단독 실행 파일
   - `windows-installer` - NSIS 설치 파일

---

## 🎯 방법 2: 수동 빌드 트리거

코드를 수정하지 않고 빌드만 다시 실행하려면:

1. GitHub 저장소 → Actions 탭
2. 좌측 "Build Windows EXE" 클릭
3. 우측 "Run workflow" 버튼 클릭
4. "Run workflow" 확인

---

## 🎯 방법 3: 릴리즈 생성 (버전 관리)

정식 버전을 만들려면:

```powershell
# 버전 태그 생성
git tag v1.0.0

# 태그 푸시
git push origin v1.0.0
```

그러면:
1. 자동으로 빌드 시작
2. GitHub Releases에 자동 등록
3. EXE 파일들이 릴리즈에 첨부됨

릴리즈 확인:
- GitHub 저장소 → Releases 탭

---

## 📦 다운로드 파일

빌드 완료 후 다운로드되는 파일:

### windows-exe.zip
```
yl-marketing-control-center.exe  (~5MB)
```
- 단독 실행 파일
- 압축 해제 후 바로 실행 가능

### windows-installer.zip
```
Y.L.Marketing Control Center_1.0.0_x64-setup.exe  (~7MB)
```
- NSIS 설치 프로그램
- 더블클릭하여 설치
- 시작 메뉴에 바로가기 생성

---

## 🔧 빌드 설정 변경

### 버전 변경

빌드 전에 버전을 변경하려면:

1. `package.json` 수정:
```json
{
  "version": "1.0.1"
}
```

2. `src-tauri/Cargo.toml` 수정:
```toml
[package]
version = "1.0.1"
```

3. `src-tauri/tauri.conf.json` 수정:
```json
{
  "version": "1.0.1"
}
```

4. 커밋 후 푸시:
```powershell
git add .
git commit -m "Bump version to 1.0.1"
git push
```

### 앱 이름 변경

`src-tauri/tauri.conf.json`:
```json
{
  "productName": "새로운 이름"
}
```

---

## 🐛 문제 해결

### 빌드 실패

1. Actions 탭에서 실패한 워크플로우 클릭
2. 빨간색 X 표시된 단계 클릭
3. 에러 로그 확인

일반적인 원인:
- `package.json`에 문법 오류
- `Cargo.toml`에 문법 오류
- 의존성 버전 충돌

### Artifacts 다운로드 안 됨

- 빌드가 완전히 완료되었는지 확인 (초록색 체크)
- 24시간 이내에 다운로드 (자동 삭제됨)

### Private 저장소 제한

- GitHub Free: 월 2,000분 무료
- 빌드 1회당 약 10-15분 소요
- 월 약 130회 빌드 가능

---

## 💡 팁

### 빌드 시간 단축

`.github/workflows/build.yml`에 캐시 추가:

```yaml
- name: Cache Rust
  uses: actions/cache@v3
  with:
    path: |
      ~/.cargo/bin/
      ~/.cargo/registry/index/
      ~/.cargo/registry/cache/
      ~/.cargo/git/db/
      src-tauri/target/
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
```

### 자동 버전 증가

`package.json`에 스크립트 추가:
```json
{
  "scripts": {
    "version:patch": "npm version patch && git push && git push --tags",
    "version:minor": "npm version minor && git push && git push --tags",
    "version:major": "npm version major && git push && git push --tags"
  }
}
```

사용:
```powershell
npm run version:patch  # 1.0.0 → 1.0.1
```

---

## 🔒 보안

### Private 저장소 권장

코드가 공개되지 않도록:
1. 저장소 생성 시 "Private" 선택
2. 또는 기존 저장소: Settings → Danger Zone → Change visibility

### Secrets 관리

민감한 정보는 GitHub Secrets에 저장:
1. 저장소 → Settings → Secrets and variables → Actions
2. "New repository secret" 클릭
3. 워크플로우에서 `${{ secrets.SECRET_NAME }}` 사용

---

## 📞 지원

문제가 발생하면:
1. Actions 탭에서 빌드 로그 확인
2. 에러 메시지 복사
3. GitHub Issues에 등록

---

## 🎉 완료!

이제 로컬 환경 설정 없이도 GitHub에서 자동으로 Windows EXE를 빌드할 수 있습니다!

**빌드 시간**: 약 10-15분  
**비용**: 무료 (월 2,000분 제한)  
**결과물**: EXE + 인스톨러
