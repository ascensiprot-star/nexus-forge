"""Database Architect agent - designs schemas, indexing, and storage strategies."""

from __future__ import annotations

from typing import Any

from forge_agents.base import AgentAction, AgentRole, BaseAgent, TaskContext


class DatabaseArchitectAgent(BaseAgent):
    @property
    def role(self) -> AgentRole:
        return AgentRole.DATABASE_ARCHITECT

    @property
    def capabilities(self) -> list[str]:
        return ["schema_design", "migration_design", "indexing_strategy", "query_optimization"]

    @property
    def system_prompt(self) -> str:
        return "You are a database architect. You design efficient schemas, indexing strategies, and migrations."

    async def observe(self, context: TaskContext) -> dict[str, Any]:
        return {"task": context.task_description}

    async def think(self, context: TaskContext, observations: dict[str, Any]) -> list[dict[str, Any]]:
        return [{"step": "design_schema", "description": "Design database schema"}]

    async def act(self, context: TaskContext, plan: list[dict[str, Any]]) -> list[AgentAction]:
        return [AgentAction(action_type="schema_design", description="Designed schema", parameters={"task_id": context.task_id})]
