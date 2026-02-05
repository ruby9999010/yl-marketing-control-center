import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { Loader2, Server, AlertCircle } from 'lucide-react';
import { TitleBar } from './components/TitleBar';
import { ClientList } from './components/ClientList';
import { LogViewer } from './components/LogViewer';
import { MemoPanel } from './components/MemoPanel';
import { Client, ClientStatus, ClientMemo, LogEntry, ClientWithStatus } from './types';
import './App.css';

function App() {
  const [isInitializing, setIsInitializing] = useState(true);
  const [serverStarted, setServerStarted] = useState(false);
  const [serverPort] = useState(9999); // setServerPort 제거
  const [error, setError] = useState<string | null>(null);

  const [clients, setClients] = useState<Client[]>([]);
  const [connectedClientIds, setConnectedClientIds] = useState<string[]>([]);
  const [clientStatuses, setClientStatuses] = useState<Map<string, ClientStatus>>(new Map());
  const [selectedClientId, setSelectedClientId] = useState<string | null>(null);
  const [selectedMemo, setSelectedMemo] = useState<ClientMemo | null>(null);
  const [logs, setLogs] = useState<LogEntry[]>([]);

  useEffect(() => {
    initApp();
  }, []);

  useEffect(() => {
    if (!serverStarted) return;

    // 이벤트 리스너 설정
    const unlistenPromises = [
      listen('client-connected', (event: any) => {
        console.log('Client connected:', event.payload);
        loadClients();
        loadConnectedClients();
      }),
      listen('client-disconnected', (event: any) => {
        console.log('Client disconnected:', event.payload);
        loadConnectedClients();
      }),
      listen('client-status-update', (event: any) => {
        const { clientId, data } = event.payload;
        setClientStatuses((prev) => new Map(prev).set(clientId, data));
      }),
      listen('client-log', (event: any) => {
        const { clientId, level, message, metadata } = event.payload;
        const newLog: LogEntry = {
          id: Date.now(),
          clientId,
          level,
          message,
          timestamp: new Date().toISOString(),
          metadata: metadata ? JSON.stringify(metadata) : null,
        };
        setLogs((prev) => [...prev, newLog].slice(-1000)); // 최근 1000개만 유지
      }),
    ];

    return () => {
      Promise.all(unlistenPromises).then((unlisteners) => {
        unlisteners.forEach((unlisten) => unlisten());
      });
    };
  }, [serverStarted]);

  useEffect(() => {
    if (selectedClientId) {
      loadMemo(selectedClientId);
    } else {
      setSelectedMemo(null);
    }
  }, [selectedClientId]);

  async function initApp() {
    try {
      // DB 초기화
      await invoke('init_db');

      // 클라이언트 목록 로드
      await loadClients();

      // WebSocket 서버 시작
      await invoke('start_websocket_server', { port: serverPort });
      setServerStarted(true);

      // 연결된 클라이언트 확인
      await loadConnectedClients();

      // 로그 로드
      await loadLogs();
    } catch (error) {
      console.error('Failed to initialize app:', error);
      setError(String(error));
    } finally {
      setIsInitializing(false);
    }
  }

  async function loadClients() {
    try {
      const result = await invoke<Client[]>('list_clients');
      setClients(result);

      // 각 클라이언트의 상태 로드
      const statuses = await invoke<ClientStatus[]>('list_client_statuses');
      const statusMap = new Map<string, ClientStatus>();
      statuses.forEach((status) => {
        statusMap.set(status.clientId, status);
      });
      setClientStatuses(statusMap);
    } catch (error) {
      console.error('Failed to load clients:', error);
    }
  }

  async function loadConnectedClients() {
    try {
      const result = await invoke<string[]>('get_connected_clients');
      setConnectedClientIds(result);
    } catch (error) {
      console.error('Failed to load connected clients:', error);
    }
  }

  async function loadMemo(clientId: string) {
    try {
      const result = await invoke<ClientMemo | null>('get_client_memo', { clientId });
      setSelectedMemo(result);
    } catch (error) {
      console.error('Failed to load memo:', error);
    }
  }

  async function loadLogs() {
    try {
      const result = await invoke<LogEntry[]>('get_logs', {
        clientId: null,
        limit: 1000,
      });
      setLogs(result.reverse()); // 최신순으로
    } catch (error) {
      console.error('Failed to load logs:', error);
    }
  }

  async function handleSaveMemo(clientId: string, memo: string) {
    try {
      await invoke('save_client_memo', { clientId, memo });
      await loadMemo(clientId);
    } catch (error) {
      console.error('Failed to save memo:', error);
      throw error;
    }
  }

  async function handleClearLogs() {
    if (!confirm('모든 로그를 삭제하시겠습니까?')) return;

    try {
      await invoke('clear_old_logs', { days: 0 });
      setLogs([]);
    } catch (error) {
      console.error('Failed to clear logs:', error);
    }
  }

  async function handleExportLogs() {
    // 향후 구현: 로그를 파일로 내보내기
    console.log('Export logs');
  }

  const clientsWithStatus: ClientWithStatus[] = clients.map((client) => ({
    ...client,
    status: clientStatuses.get(client.clientId),
    isConnected: connectedClientIds.includes(client.clientId),
  }));

  if (isInitializing) {
    return (
      <div className="h-screen flex items-center justify-center bg-macos-bg">
        <Loader2 className="w-8 h-8 animate-spin text-macos-blue" />
      </div>
    );
  }

  if (error) {
    return (
      <div className="h-screen flex items-center justify-center bg-macos-bg">
        <div className="w-[400px] p-8 bg-white/5 border border-white/10 rounded-macos-lg text-center">
          <AlertCircle className="w-12 h-12 text-macos-red mx-auto mb-4" />
          <h2 className="text-[17px] font-semibold text-macos-text-primary mb-2">
            초기화 실패
          </h2>
          <p className="text-[13px] text-macos-text-secondary mb-4">
            {error}
          </p>
          <button
            onClick={() => window.location.reload()}
            className="px-4 py-2 bg-macos-blue text-white rounded-macos text-[13px] font-medium hover:bg-macos-blue/90 transition-colors"
          >
            다시 시도
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="h-screen flex flex-col bg-macos-bg">
      <TitleBar />

      {/* 서버 상태 바 */}
      <div className="px-4 py-2 bg-macos-blue/10 border-b border-macos-blue/20 flex items-center justify-between">
        <div className="flex items-center gap-2">
          <Server className="w-4 h-4 text-macos-blue" />
          <span className="text-[12px] text-macos-blue font-medium">
            WebSocket 서버 실행 중
          </span>
          <span className="text-[11px] text-macos-text-tertiary">
            포트: {serverPort}
          </span>
        </div>
        <div className="flex items-center gap-4 text-[11px]">
          <span className="text-macos-text-secondary">
            연결: <span className="text-macos-green font-medium">{connectedClientIds.length}</span>
          </span>
          <span className="text-macos-text-secondary">
            등록: <span className="text-macos-text-primary font-medium">{clients.length}</span>
          </span>
          <span className="text-macos-text-secondary">
            로그: <span className="text-macos-text-primary font-medium">{logs.length}</span>
          </span>
        </div>
      </div>

      {/* 메인 콘텐츠 */}
      <div className="flex-1 flex overflow-hidden">
        {/* 좌측: 클라이언트 목록 */}
        <div className="w-80">
          <ClientList
            clients={clientsWithStatus}
            selectedClientId={selectedClientId}
            onSelectClient={setSelectedClientId}
          />
        </div>

        {/* 중앙: 로그 뷰어 */}
        <div className="flex-1">
          <LogViewer
            logs={logs}
            selectedClientId={selectedClientId}
            onClearLogs={handleClearLogs}
            onExportLogs={handleExportLogs}
          />
        </div>

        {/* 우측: 메모 패널 */}
        <div className="w-96">
          <MemoPanel
            clientId={selectedClientId}
            memo={selectedMemo}
            onSaveMemo={handleSaveMemo}
          />
        </div>
      </div>
    </div>
  );
}

export default App;
