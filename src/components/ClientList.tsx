import { Wifi, WifiOff, Circle } from 'lucide-react';
import { ClientWithStatus } from '../types';
import { formatDistanceToNow } from '../utils/dateUtils';

interface ClientListProps {
  clients: ClientWithStatus[];
  selectedClientId: string | null;
  onSelectClient: (clientId: string) => void;
}

export function ClientList({ clients, selectedClientId, onSelectClient }: ClientListProps) {
  const getStatusColor = (status?: string) => {
    switch (status) {
      case 'running': return 'text-macos-blue';
      case 'idle': return 'text-macos-green';
      case 'error': return 'text-macos-red';
      case 'paused': return 'text-macos-yellow';
      default: return 'text-macos-text-tertiary';
    }
  };

  const getStatusText = (status?: string) => {
    switch (status) {
      case 'running': return '실행 중';
      case 'idle': return '대기';
      case 'error': return '오류';
      case 'paused': return '일시정지';
      default: return '알 수 없음';
    }
  };

  const formatNextExecution = (seconds: number) => {
    if (seconds <= 0) return '대기 중';
    
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;

    if (hours > 0) return `${hours}시간 ${minutes}분`;
    if (minutes > 0) return `${minutes}분 ${secs}초`;
    return `${secs}초`;
  };

  return (
    <div className="flex flex-col h-full bg-macos-bg border-r border-macos-separator">
      <div className="px-4 py-3 border-b border-macos-separator">
        <h2 className="text-[13px] font-semibold text-macos-text-primary">
          클라이언트 ({clients.length})
        </h2>
      </div>

      <div className="flex-1 overflow-y-auto">
        {clients.length === 0 ? (
          <div className="flex flex-col items-center justify-center h-full p-6 text-center">
            <WifiOff className="w-12 h-12 text-macos-text-tertiary mb-3" />
            <p className="text-[13px] text-macos-text-secondary">
              연결된 클라이언트가 없습니다
            </p>
            <p className="text-[11px] text-macos-text-tertiary mt-1">
              클라이언트에서 관제 센터를 활성화하세요
            </p>
          </div>
        ) : (
          <div className="p-2 space-y-1">
            {clients.map((client) => (
              <button
                key={client.clientId}
                onClick={() => onSelectClient(client.clientId)}
                className={`w-full p-3 rounded-macos text-left transition-all ${
                  selectedClientId === client.clientId
                    ? 'bg-macos-blue/20 border border-macos-blue/30'
                    : 'bg-white/5 border border-transparent hover:bg-white/10'
                }`}
              >
                <div className="flex items-start justify-between mb-2">
                  <div className="flex items-center gap-2">
                    {client.isConnected ? (
                      <Wifi className="w-4 h-4 text-macos-green" />
                    ) : (
                      <WifiOff className="w-4 h-4 text-macos-text-tertiary" />
                    )}
                    <span className="text-[13px] font-medium text-macos-text-primary font-mono">
                      {client.clientId}
                    </span>
                  </div>
                  <Circle
                    className={`w-2 h-2 fill-current ${
                      client.isConnected ? 'text-macos-green' : 'text-macos-text-tertiary'
                    }`}
                  />
                </div>

                {client.status && (
                  <div className="space-y-1">
                    <div className="flex items-center justify-between text-[11px]">
                      <span className="text-macos-text-secondary">상태</span>
                      <span className={getStatusColor(client.status.currentStatus)}>
                        {getStatusText(client.status.currentStatus)}
                      </span>
                    </div>
                    <div className="flex items-center justify-between text-[11px]">
                      <span className="text-macos-text-secondary">성공</span>
                      <span className="text-macos-green">{client.status.successCount}</span>
                    </div>
                    <div className="flex items-center justify-between text-[11px]">
                      <span className="text-macos-text-secondary">실패</span>
                      <span className="text-macos-red">{client.status.failureCount}</span>
                    </div>
                    {client.status.nextExecutionIn > 0 && (
                      <div className="flex items-center justify-between text-[11px]">
                        <span className="text-macos-text-secondary">다음 실행</span>
                        <span className="text-macos-text-primary">
                          {formatNextExecution(client.status.nextExecutionIn)}
                        </span>
                      </div>
                    )}
                  </div>
                )}

                {client.lastSeen && (
                  <div className="mt-2 pt-2 border-t border-white/5">
                    <p className="text-[10px] text-macos-text-tertiary">
                      마지막 접속: {formatDistanceToNow(client.lastSeen)}
                    </p>
                  </div>
                )}
              </button>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
