import os
from dataclasses import dataclass

@dataclass(frozen=True)
class Policy:
    # Defaults aligned to P3-04 §1.2
    ALLOW_ZERO_TIMESTAMP_BUCKET: bool = True
    timestamp_window_enforced: bool = False

    @staticmethod
    def from_env() -> "Policy":
        def _b(name: str, default: bool) -> bool:
            v = os.environ.get(name)
            if v is None:
                return default
            return v.strip().lower() in ("1","true","yes","on")
        return Policy(
            ALLOW_ZERO_TIMESTAMP_BUCKET=_b("QSHIELD_ALLOW_ZERO_TIMESTAMP_BUCKET", True),
            timestamp_window_enforced=_b("QSHIELD_TIMESTAMP_WINDOW_ENFORCED", False),
        )

    def satisfies(self, requires: dict) -> bool:
        for k, v in requires.items():
            if not hasattr(self, k):
                return False
            if getattr(self, k) != v:
                return False
        return True

    def to_dict(self) -> dict:
        return {
            "ALLOW_ZERO_TIMESTAMP_BUCKET": self.ALLOW_ZERO_TIMESTAMP_BUCKET,
            "timestamp_window_enforced": self.timestamp_window_enforced,
        }
