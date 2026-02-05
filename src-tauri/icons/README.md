# 아이콘 파일

Tauri 빌드를 위해서는 아이콘 파일이 필요합니다.

## 임시 해결책

현재 `tauri.conf.json`에서 아이콘 설정을 제거하여 기본 아이콘을 사용합니다.

## 커스텀 아이콘 추가 방법

1. **아이콘 이미지 준비** (PNG, 1024x1024 권장)

2. **온라인 도구 사용**
   - https://icon.kitchen/ 접속
   - 이미지 업로드
   - "Tauri" 선택
   - 다운로드

3. **생성된 파일 복사**
   ```
   icons/
   ├── 32x32.png
   ├── 128x128.png
   ├── 128x128@2x.png
   ├── icon.icns (macOS)
   └── icon.ico (Windows)
   ```

4. **tauri.conf.json 수정**
   ```json
   "bundle": {
     "icon": [
       "icons/32x32.png",
       "icons/128x128.png",
       "icons/128x128@2x.png",
       "icons/icon.icns",
       "icons/icon.ico"
     ]
   }
   ```

## 현재 상태

아이콘 없이 빌드하면 Tauri 기본 아이콘이 사용됩니다.
