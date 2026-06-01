"""EdisonDB Python SDK — Exceptions"""


class EdisonDBError(Exception):
    """Base exception for all EdisonDB errors."""
    pass

class NotFoundError(EdisonDBError):
    """Record not found."""
    pass

class AccessDeniedError(EdisonDBError):
    """Access denied — not the record owner."""
    pass

class AlreadyExistsError(EdisonDBError):
    """Record with this ID already exists."""
    pass

class AuditChainError(EdisonDBError):
    """Audit chain integrity violation detected."""
    pass

class AuthError(EdisonDBError):
    """Authentication failed — wrong owner ID or password."""
    pass
