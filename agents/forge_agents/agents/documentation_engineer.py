"""Documentation Engineer agent - generates and maintains documentation."""

from __future__ import annotations

from typing import Any

from forge_agents.base import AgentAction, AgentRole, BaseAgent, TaskContext


class DocumentationEngineerAgent(BaseAgent):
    @property
    def role(self) -> AgentRole:
        return AgentRole.DOCUMENTATION_ENGINEER

    @property
    def capabilities(self) -> list[str]:
        return ["api_docs", "architecture_docs", "user_guides", "inline_docs"]

    @property
    def system_prompt(self) -> str:
        return "You are a documentation engineer. You write clear, accurate, maintainable documentation."

    async def observe(self, context: TaskContext) -> dict[str, Any]:
        return {"task": context.task_description}

    async def think(self, context: TaskContext, observations: dict[str, Any]) -> list[dict[str, Any]]:
        return [{"step": "write_docs", "description": "Write documentation"}]

    async def act(self, context: TaskContext, plan: list[dict[str, Any]]) -> list[AgentAction]:
        return [AgentAction(action_type="documentation", description="Generated documentation", parameters={"task_id": context.task_id})]
