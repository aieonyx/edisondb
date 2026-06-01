// EdisonDB TypeScript SDK — Types

export interface EdisonRecord {
  id: string;
  tier: string;
  payload: string;
  createdAt: number;
}

export interface AuditEntry {
  recordId: string;
  requesterId: string;
  action: string;
  timestamp: number;
}

export interface DbStatus {
  recordCount: number;
  auditCount: number;
  criticalCount: number;
  personalCount: number;
  noiseCount: number;
  chainValid: boolean;
  backend: string;
}

export interface EdisonDBConfig {
  host?: string;
  port?: number;
  ownerId: string;
  password: string;
  timeoutMs?: number;
}

export interface ListOptions {
  tier?: string;
}
