// EdisonDB TypeScript SDK — Client

import {
  EdisonDBError,
  NotFoundError,
  AccessDeniedError,
  AlreadyExistsError,
  AuthError,
} from './errors';
import type {
  EdisonRecord,
  AuditEntry,
  DbStatus,
  EdisonDBConfig,
  ListOptions,
} from './types';

/**
 * EdisonDB TypeScript client.
 *
 * Connects to a running EdisonDB REST server and provides a clean
 * async API for sovereign encrypted data operations.
 *
 * @example
 * const db = new EdisonDB({ host: 'localhost', port: 7777, ownerId: 'alice', password: 'secret' });
 * await db.write('note:1', 'PERSONAL', 'my sovereign note');
 * const record = await db.read('note:1');
 */
export class EdisonDB {
  private readonly base: string;
  private readonly ownerId: string;
  private readonly password: string;
  private readonly timeoutMs: number;

  constructor(config: EdisonDBConfig) {
    const host = config.host ?? 'localhost';
    const port = config.port ?? 7777;
    this.base     = `http://${host}:${port}`;
    this.ownerId  = config.ownerId;
    this.password = config.password;
    this.timeoutMs = config.timeoutMs ?? 10000;
  }

  // -- Internal helpers -----------------------------------------------------

  private headers(): { [key: string]: string } {
    return {
      'Content-Type': 'application/json',
      'X-Owner-ID':   this.ownerId,
      'X-Password':   this.password,
    };
  }

  private async request<T>(
    method: string,
    path: string,
    body?: unknown,
  ): Promise<T> {
    const controller = new AbortController();
    const timer = setTimeout(() => controller.abort(), this.timeoutMs);
    try {
      const res = await fetch(`${this.base}${path}`, {
        method,
        headers: this.headers(),
        body: body ? JSON.stringify(body) : undefined,
        signal: controller.signal,
      });
      const data = await res.json() as { [key: string]: unknown };
      if (!res.ok) {
        const msg = (data['error'] as string) ?? res.statusText;
        if (res.status === 401) throw new AuthError(msg);
        if (res.status === 404 || msg.toLowerCase().includes('not found')) throw new NotFoundError(msg);
        if (msg.toLowerCase().includes('access denied')) throw new AccessDeniedError(msg);
        if (msg.toLowerCase().includes('already exists')) throw new AlreadyExistsError(msg);
        throw new EdisonDBError(msg);
      }
      return data as T;
    } catch (err) {
      if (err instanceof EdisonDBError) throw err;
      throw new EdisonDBError(String(err));
    } finally {
      clearTimeout(timer);
    }
  }

  // -- Public API -----------------------------------------------------------

  /** Check if the server is reachable. */
  async ping(): Promise<boolean> {
    try {
      const res = await fetch(`${this.base}/health`);
      const data = await res.json() as { status: string };
      return data.status === 'ok';
    } catch {
      return false;
    }
  }

  /**
   * Write a new encrypted record.
   * @param id      Unique record identifier (e.g. "user:1")
   * @param tier    Data tier: "CRITICAL", "PERSONAL", or "NOISE"
   * @param payload Plaintext data to encrypt and store
   */
  async write(id: string, tier: string, payload: string): Promise<void> {
    await this.request('POST', '/api/write', { id, tier, payload });
  }

  /**
   * Read and decrypt a record by ID.
   * Returns null if the record does not exist.
   */
  async read(id: string): Promise<EdisonRecord | null> {
    try {
      const data = await this.request<{
        id: string; tier: string; payload: string; created_at: number;
      }>('GET', `/api/read/${id}`);
      return {
        id:        data.id,
        tier:      data.tier,
        payload:   data.payload,
        createdAt: data.created_at,
      };
    } catch (err) {
      if (err instanceof NotFoundError) return null;
      throw err;
    }
  }

  /**
   * List all records owned by this client.
   * @param options Optional tier filter
   */
  async list(options: ListOptions = {}): Promise<EdisonRecord[]> {
    const path = options.tier
      ? `/api/list?tier=${options.tier.toUpperCase()}`
      : '/api/list';
    const data = await this.request<{ records: Array<{
      id: string; tier: string; created_at: number;
    }> }>('GET', path);
    return ((data.records as Array<{id: string; tier: string; created_at: number}>) ?? []).map(r => ({
      id:        r.id,
      tier:      r.tier,
      payload:   '',
      createdAt: r.created_at,
    }));
  }

  /** Delete a record by ID. */
  async delete(id: string): Promise<void> {
    await this.request('DELETE', `/api/delete/${id}`);
  }

  /**
   * Retrieve the audit log.
   * @param id Optional record ID filter
   */
  async audit(id?: string): Promise<AuditEntry[]> {
    const path = id ? `/api/audit?id=${id}` : '/api/audit';
    const data = await this.request<{ entries: Array<{
      record_id: string; requester_id: string; action: string; timestamp: number;
    }> }>('GET', path);
    return (data.entries ?? []).map(e => ({
      recordId:    e.record_id,
      requesterId: e.requester_id,
      action:      e.action,
      timestamp:   e.timestamp,
    }));
  }

  /** Return database statistics. */
  async status(): Promise<DbStatus> {
    const data = await this.request<{
      record_count: number; audit_count: number;
      critical_count: number; personal_count: number; noise_count: number;
      chain_valid: boolean; backend: string;
    }>('GET', '/api/status');
    return {
      recordCount:   data.record_count,
      auditCount:    data.audit_count,
      criticalCount: data.critical_count,
      personalCount: data.personal_count,
      noiseCount:    data.noise_count,
      chainValid:    data.chain_valid,
      backend:       data.backend,
    };
  }

  /**
   * Verify audit chain integrity.
   * Returns true if the chain is intact.
   */
  async verify(): Promise<boolean> {
    const data = await this.request<{ chain_valid: boolean }>('GET', '/api/verify');
    return data.chain_valid ?? false;
  }
}
