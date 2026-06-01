# EdisonDB TypeScript SDK

The official TypeScript/JavaScript client for EdisonDB.

## Install

```bash
npm install edisondb
```

## Quick Start

```typescript
import { EdisonDB } from 'edisondb';

const db = new EdisonDB({
  host: 'localhost',
  port: 7777,
  ownerId: 'alice',
  password: 'secret',
});

await db.write('user:1', 'PERSONAL', 'sovereign data');
const record = await db.read('user:1');
console.log(record?.payload); // sovereign data

const records = await db.list({ tier: 'PERSONAL' });
await db.delete('user:1');

const status = await db.status();
console.log(status.chainValid); // true
```

## Requirements

- Node.js 18+
- A running EdisonDB server (`edisondb-server --db myapp.redb --port 7777`)

## License

Apache 2.0 — © 2026 AIEONYX / Edison Lepiten
