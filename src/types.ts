export interface Client {
  id: number;
  clientId: string;
  displayName?: string | null;
  registeredAt: string;
  lastSeen?: string | null;
  isActive: number;
}

export interface ClientStatus {
  clientId: string;
  successCount: number;
  failureCount: number;
  nextExecutionIn: number;
  currentStatus: "idle" | "running" | "error" | "paused" | "unknown";
  lastActivity?: string | null;
  updatedAt: string;
}

export interface ClientMemo {
  id: number;
  clientId: string;
  memo: string;
  updatedAt: string;
}

export interface LogEntry {
  id: number;
  clientId: string;
  level: "info" | "success" | "error" | "warning";
  message: string;
  timestamp: string;
  metadata?: string | null;
}

export interface ClientWithStatus extends Client {
  status?: ClientStatus;
  isConnected: boolean;
}
