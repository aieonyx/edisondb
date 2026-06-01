// EdisonDB TypeScript SDK — Integration Tests
// Requires: edisondb-server --db /tmp/ts_test.redb --port 7779

import { EdisonDB, NotFoundError, AlreadyExistsError } from '../src/index';

const config = { host: 'localhost', port: 7779, ownerId: 'alice', password: 'password' };

let db: EdisonDB;

beforeEach(() => {
  db = new EdisonDB(config);
});

test('ping returns true', async () => {
  expect(await db.ping()).toBe(true);
});

test('write and read', async () => {
  await db.write('ts:1', 'PERSONAL', 'sovereign typescript data');
  const rec = await db.read('ts:1');
  expect(rec).not.toBeNull();
  expect(rec!.payload).toBe('sovereign typescript data');
  expect(rec!.tier).toBe('personal');
});

test('read nonexistent returns null', async () => {
  expect(await db.read('ts:ghost')).toBeNull();
});

test('duplicate write throws AlreadyExistsError', async () => {
  await db.write('ts:dup', 'NOISE', 'first');
  await expect(db.write('ts:dup', 'NOISE', 'second')).rejects.toThrow(AlreadyExistsError);
});

test('delete removes record', async () => {
  await db.write('ts:del', 'NOISE', 'to delete');
  await db.delete('ts:del');
  expect(await db.read('ts:del')).toBeNull();
});

test('list returns all records', async () => {
  await db.write('ts:la1', 'CRITICAL', 'a');
  await db.write('ts:la2', 'NOISE', 'b');
  const records = await db.list();
  const ids = records.map(r => r.id);
  expect(ids).toContain('ts:la1');
  expect(ids).toContain('ts:la2');
});

test('list with tier filter', async () => {
  await db.write('ts:tf1', 'CRITICAL', 'secret');
  await db.write('ts:tf2', 'NOISE', 'log');
  const critical = await db.list({ tier: 'CRITICAL' });
  expect(critical.every(r => r.tier === 'critical')).toBe(true);
});

test('status returns correct counts', async () => {
  const s = await db.status();
  expect(s.recordCount).toBeGreaterThanOrEqual(0);
  expect(s.chainValid).toBe(true);
  expect(s.backend).toBe('redb');
});

test('verify returns true', async () => {
  expect(await db.verify()).toBe(true);
});

test('audit returns entries', async () => {
  await db.write('ts:audit', 'PERSONAL', 'data');
  const entries = await db.audit('ts:audit');
  expect(entries.length).toBeGreaterThanOrEqual(1);
  expect(entries[0].recordId).toBe('ts:audit');
});
