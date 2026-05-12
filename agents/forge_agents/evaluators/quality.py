"""Quality evaluation for agent-generated code."""

from __future__ import annotations

from pydantic import BaseModel


class QualityScore(BaseModel):
    correctness: float = 0.0
    completeness: float = 0.0
    code_quality: float = 0.0
    test_coverage: float = 0.0
    security: float = 0.0
    overall: float = 0.0

    def compute_overall(self) -> float:
        weights = {
            "correctness": 0.3,
            "completeness": 0.2,
            "code_quality": 0.2,
            "test_coverage": 0.15,
            "security": 0.15,
        }
        self.overall = sum(
            getattr(self, k) * v for k, v in weights.items()
        )
        return self.overall

    def passes_threshold(self, threshold: float = 0.7) -> bool:
        return self.compute_overall() >= threshold
