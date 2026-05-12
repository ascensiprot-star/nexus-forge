"""Software Engineer agent - implements core application logic and services."""

from __future__ import annotations

from typing import Any

from forge_agents.base import AgentAction, AgentRole, BaseAgent, TaskContext


class SoftwareEngineerAgent(BaseAgent):
    @property
    def role(self) -> AgentRole:
        return AgentRole.SOFTWARE_ENGINEER

    @property
    def capabilities(self) -> list[str]:
        return [
            "code_generation",
            "code_modification",
            "refactoring",
            "bug_fixing",
            "api_implementation",
            "service_implementation",
        ]

    @property
    def system_prompt(self) -> str:
        return (
            "You are a senior software engineer. You write clean, maintainable, "
            "production-quality code. You follow established patterns and conventions "
            "in the existing codebase. You write comprehensive tests for your code. "
            "You consider edge cases, error handling, and performance."
        )

    async def observe(self, context: TaskContext) -> dict[str, Any]:
        return {
            "task": context.task_description,
            "files": list(context.file_context.keys()),
            "constraints": context.constraints,
            "existing_patterns": [],
        }

    async def think(
        self, context: TaskContext, observations: dict[str, Any]
    ) -> list[dict[str, Any]]:
        return [
            {
                "step": "analyze",
                "description": "Analyze requirements and existing code",
            },
            {
                "step": "implement",
                "description": "Write implementation code",
            },
            {
                "step": "test",
                "description": "Write tests for the implementation",
            },
        ]

    async def act(
        self, context: TaskContext, plan: list[dict[str, Any]]
    ) -> list[AgentAction]:
        actions = []
        for step in plan:
            action = AgentAction(
                action_type=step["step"],
                description=step["description"],
                parameters={"task_id": context.task_id},
            )
            actions.append(action)
        return actions
