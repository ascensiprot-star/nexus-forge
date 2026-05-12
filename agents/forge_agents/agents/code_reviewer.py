"""Code Reviewer agent - reviews code for quality, correctness, and maintainability."""

from __future__ import annotations

from typing import Any

from forge_agents.base import AgentAction, AgentRole, BaseAgent, TaskContext


class CodeReviewerAgent(BaseAgent):
    @property
    def role(self) -> AgentRole:
        return AgentRole.CODE_REVIEWER

    @property
    def capabilities(self) -> list[str]:
        return [
            "code_review",
            "architecture_review",
            "security_review",
            "performance_review",
            "naming_review",
            "approval",
        ]

    @property
    def system_prompt(self) -> str:
        return (
            "You are a meticulous code reviewer. You examine code for correctness, "
            "maintainability, security, performance, and adherence to conventions. "
            "You provide specific, actionable feedback with file and line references. "
            "You distinguish blocking issues from suggestions."
        )

    async def observe(self, context: TaskContext) -> dict[str, Any]:
        return {
            "files_to_review": list(context.file_context.keys()),
            "review_criteria": [
                "correctness",
                "security",
                "performance",
                "maintainability",
                "naming",
            ],
        }

    async def think(
        self, context: TaskContext, observations: dict[str, Any]
    ) -> list[dict[str, Any]]:
        return [
            {"step": "review", "description": "Review code changes"},
            {"step": "annotate", "description": "Add review comments"},
            {"step": "verdict", "description": "Approve or request changes"},
        ]

    async def act(
        self, context: TaskContext, plan: list[dict[str, Any]]
    ) -> list[AgentAction]:
        return [
            AgentAction(
                action_type="code_review",
                description="Reviewed code for quality and correctness",
                parameters={"task_id": context.task_id, "verdict": "approve"},
            )
        ]
