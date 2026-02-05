import { useState, useEffect } from 'react';
import { Save, Loader2, Check } from 'lucide-react';
import { ClientMemo } from '../types';

interface MemoPanelProps {
  clientId: string | null;
  memo: ClientMemo | null;
  onSaveMemo: (clientId: string, memo: string) => Promise<void>;
}

export function MemoPanel({ clientId, memo, onSaveMemo }: MemoPanelProps) {
  const [memoText, setMemoText] = useState('');
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);
  const [hasChanges, setHasChanges] = useState(false);

  useEffect(() => {
    if (memo) {
      setMemoText(memo.memo);
      setHasChanges(false);
    } else {
      setMemoText('');
      setHasChanges(false);
    }
    setSaved(false);
  }, [memo, clientId]);

  const handleSave = async () => {
    if (!clientId || !hasChanges) return;

    setSaving(true);
    try {
      await onSaveMemo(clientId, memoText);
      setHasChanges(false);
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } catch (error) {
      console.error('Failed to save memo:', error);
    } finally {
      setSaving(false);
    }
  };

  const handleChange = (value: string) => {
    setMemoText(value);
    setHasChanges(value !== (memo?.memo || ''));
    setSaved(false);
  };

  // 자동 저장 (3초 후)
  useEffect(() => {
    if (!hasChanges || !clientId) return;

    const timer = setTimeout(() => {
      handleSave();
    }, 3000);

    return () => clearTimeout(timer);
  }, [memoText, hasChanges, clientId]);

  if (!clientId) {
    return (
      <div className="flex flex-col h-full bg-macos-bg border-l border-macos-separator">
        <div className="px-4 py-3 border-b border-macos-separator">
          <h2 className="text-[13px] font-semibold text-macos-text-primary">메모</h2>
        </div>
        <div className="flex-1 flex items-center justify-center p-6 text-center">
          <p className="text-[13px] text-macos-text-secondary">
            클라이언트를 선택하면<br />메모를 작성할 수 있습니다
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-macos-bg border-l border-macos-separator">
      {/* 헤더 */}
      <div className="px-4 py-3 border-b border-macos-separator">
        <div className="flex items-center justify-between">
          <h2 className="text-[13px] font-semibold text-macos-text-primary">메모</h2>
          <div className="flex items-center gap-2">
            {saved && (
              <span className="flex items-center gap-1 text-[11px] text-macos-green">
                <Check className="w-3 h-3" />
                저장됨
              </span>
            )}
            {hasChanges && !saving && (
              <span className="text-[11px] text-macos-text-tertiary">
                저장 안 됨
              </span>
            )}
            <button
              onClick={handleSave}
              disabled={!hasChanges || saving}
              className="p-1.5 rounded hover:bg-white/10 transition-colors text-macos-text-secondary disabled:opacity-50 disabled:cursor-not-allowed"
              title="메모 저장"
            >
              {saving ? (
                <Loader2 className="w-4 h-4 animate-spin" />
              ) : (
                <Save className="w-4 h-4" />
              )}
            </button>
          </div>
        </div>
        <p className="text-[11px] text-macos-text-tertiary mt-1">
          {clientId}
        </p>
      </div>

      {/* 메모 입력 영역 */}
      <div className="flex-1 p-4">
        <textarea
          value={memoText}
          onChange={(e) => handleChange(e.target.value)}
          placeholder="이 클라이언트에 대한 메모를 입력하세요...&#10;&#10;예:&#10;- 서버 용도: 테스트 서버&#10;- 담당자: 홍길동&#10;- 특이사항: 주말에만 실행"
          className="w-full h-full p-3 bg-white/5 border border-white/10 rounded-macos text-[13px] text-macos-text-primary placeholder:text-macos-text-tertiary focus:outline-none focus:border-macos-blue/50 resize-none"
        />
      </div>

      {/* 푸터 */}
      <div className="px-4 py-2 border-t border-macos-separator">
        <div className="flex items-center justify-between text-[11px] text-macos-text-tertiary">
          <span>{memoText.length} 글자</span>
          {memo?.updatedAt && (
            <span>마지막 수정: {new Date(memo.updatedAt).toLocaleString('ko-KR')}</span>
          )}
        </div>
      </div>
    </div>
  );
}
