import { useEffect, useRef, useState } from 'react';
import { Search, Download, Trash2, Filter } from 'lucide-react';
import { LogEntry } from '../types';
import { formatTime } from '../utils/dateUtils';

interface LogViewerProps {
  logs: LogEntry[];
  selectedClientId: string | null;
  onClearLogs?: () => void;
  onExportLogs?: () => void;
}

export function LogViewer({ logs, selectedClientId, onClearLogs, onExportLogs }: LogViewerProps) {
  const [searchQuery, setSearchQuery] = useState('');
  const [levelFilter, setLevelFilter] = useState<string>('all');
  const logsEndRef = useRef<HTMLDivElement>(null);
  const [autoScroll, setAutoScroll] = useState(true);

  useEffect(() => {
    if (autoScroll) {
      logsEndRef.current?.scrollIntoView({ behavior: 'smooth' });
    }
  }, [logs, autoScroll]);

  const filteredLogs = logs.filter((log) => {
    const matchesSearch = searchQuery === '' || 
      log.message.toLowerCase().includes(searchQuery.toLowerCase()) ||
      log.clientId.toLowerCase().includes(searchQuery.toLowerCase());
    
    const matchesLevel = levelFilter === 'all' || log.level === levelFilter;
    
    const matchesClient = !selectedClientId || log.clientId === selectedClientId;

    return matchesSearch && matchesLevel && matchesClient;
  });

  const getLevelColor = (level: string) => {
    switch (level) {
      case 'success': return 'text-macos-green';
      case 'error': return 'text-macos-red';
      case 'warning': return 'text-macos-yellow';
      default: return 'text-macos-text-secondary';
    }
  };

  const getLevelBg = (level: string) => {
    switch (level) {
      case 'success': return 'bg-macos-green/10';
      case 'error': return 'bg-macos-red/10';
      case 'warning': return 'bg-macos-yellow/10';
      default: return 'bg-white/5';
    }
  };

  const getLevelLabel = (level: string) => {
    switch (level) {
      case 'success': return '성공';
      case 'error': return '오류';
      case 'warning': return '경고';
      default: return '정보';
    }
  };

  return (
    <div className="flex flex-col h-full bg-macos-bg">
      {/* 헤더 */}
      <div className="px-4 py-3 border-b border-macos-separator space-y-3">
        <div className="flex items-center justify-between">
          <h2 className="text-[13px] font-semibold text-macos-text-primary">
            실시간 로그 {selectedClientId && `(${selectedClientId})`}
          </h2>
          <div className="flex items-center gap-2">
            <button
              onClick={() => setAutoScroll(!autoScroll)}
              className={`px-2 py-1 text-[11px] rounded transition-colors ${
                autoScroll
                  ? 'bg-macos-blue text-white'
                  : 'bg-white/5 text-macos-text-secondary hover:bg-white/10'
              }`}
            >
              자동 스크롤
            </button>
            {onExportLogs && (
              <button
                onClick={onExportLogs}
                className="p-1.5 rounded hover:bg-white/10 transition-colors text-macos-text-secondary"
                title="로그 내보내기"
              >
                <Download className="w-4 h-4" />
              </button>
            )}
            {onClearLogs && (
              <button
                onClick={onClearLogs}
                className="p-1.5 rounded hover:bg-white/10 transition-colors text-macos-text-secondary"
                title="로그 지우기"
              >
                <Trash2 className="w-4 h-4" />
              </button>
            )}
          </div>
        </div>

        {/* 검색 및 필터 */}
        <div className="flex gap-2">
          <div className="flex-1 relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-macos-text-tertiary" />
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="로그 검색..."
              className="w-full h-8 pl-9 pr-3 bg-white/5 border border-white/10 rounded-macos text-[12px] text-macos-text-primary placeholder:text-macos-text-tertiary focus:outline-none focus:border-macos-blue/50"
            />
          </div>
          <select
            value={levelFilter}
            onChange={(e) => setLevelFilter(e.target.value)}
            className="h-8 px-3 bg-white/5 border border-white/10 rounded-macos text-[12px] text-macos-text-primary focus:outline-none focus:border-macos-blue/50"
          >
            <option value="all">전체</option>
            <option value="info">정보</option>
            <option value="success">성공</option>
            <option value="warning">경고</option>
            <option value="error">오류</option>
          </select>
        </div>
      </div>

      {/* 로그 목록 */}
      <div className="flex-1 overflow-y-auto p-4 space-y-2 font-mono text-[12px]">
        {filteredLogs.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full text-center">
            <Filter className="w-12 h-12 text-macos-text-tertiary mb-3" />
            <p className="text-[13px] text-macos-text-secondary">
              {searchQuery || levelFilter !== 'all' ? '검색 결과가 없습니다' : '로그가 없습니다'}
            </p>
          </div>
        ) : (
          <>
            {filteredLogs.map((log) => (
              <div
                key={log.id}
                className={`p-3 rounded-macos border border-white/10 ${getLevelBg(log.level)}`}
              >
                <div className="flex items-start gap-3">
                  <span className="text-[11px] text-macos-text-tertiary whitespace-nowrap">
                    {formatTime(log.timestamp)}
                  </span>
                  <span className={`text-[11px] font-semibold uppercase whitespace-nowrap ${getLevelColor(log.level)}`}>
                    [{getLevelLabel(log.level)}]
                  </span>
                  {!selectedClientId && (
                    <span className="text-[11px] text-macos-blue whitespace-nowrap">
                      {log.clientId}
                    </span>
                  )}
                  <span className="flex-1 text-macos-text-primary break-words">
                    {log.message}
                  </span>
                </div>
                {log.metadata && (
                  <div className="mt-2 pl-3 border-l-2 border-white/10">
                    <pre className="text-[10px] text-macos-text-tertiary overflow-x-auto">
                      {JSON.stringify(JSON.parse(log.metadata), null, 2)}
                    </pre>
                  </div>
                )}
              </div>
            ))}
            <div ref={logsEndRef} />
          </>
        )}
      </div>

      {/* 푸터 */}
      <div className="px-4 py-2 border-t border-macos-separator">
        <p className="text-[11px] text-macos-text-tertiary">
          총 {filteredLogs.length}개의 로그
        </p>
      </div>
    </div>
  );
}
