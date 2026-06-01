// EdisonDB TypeScript SDK — Errors

export class EdisonDBError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'EdisonDBError';
  }
}

export class NotFoundError extends EdisonDBError {
  constructor(message = 'Record not found') {
    super(message);
    this.name = 'NotFoundError';
  }
}

export class AccessDeniedError extends EdisonDBError {
  constructor(message = 'Access denied') {
    super(message);
    this.name = 'AccessDeniedError';
  }
}

export class AlreadyExistsError extends EdisonDBError {
  constructor(message = 'Record already exists') {
    super(message);
    this.name = 'AlreadyExistsError';
  }
}

export class AuthError extends EdisonDBError {
  constructor(message = 'Authentication failed') {
    super(message);
    this.name = 'AuthError';
  }
}

export class AuditChainError extends EdisonDBError {
  constructor(message = 'Audit chain integrity violation') {
    super(message);
    this.name = 'AuditChainError';
  }
}
