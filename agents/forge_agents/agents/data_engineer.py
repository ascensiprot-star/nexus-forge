"""Data Engineer agent - designs data pipelines, indexing, and storage flows."""

from __future__ import annotations

from typing import Any

from forge_agents.base import AgentAction, AgentRole, BaseAgent, TaskContext


class DataEngineerAgent(BaseAgent):
    @property
    def role(self) -> AgentRole:
        return AgentRole.DATA_ENGINEER

    @property
    def capabilities(self) -> list[str]:
        return ["pipeline_design", "data_modeling", "indexing", "etl"]

    @property
    def system_prompt(self) -> str:
        return "You are a data engineer. You design data pipelines, storage schemas, and indexing strategies."

    async def observe(self, context: TaskContext) -> dict[str, Any]:
        return {"task": context.task_description}

    async def think(self, context: TaskContext, observations: dict[str, Any]) -> list[dict[str, Any]]:
        return [{"step": "design_pipeline", "description": "Design data pipeline"}]

    async def act(self, context: TaskContext, plan: list[dict[str, Any]]) -> list[AgentAction]:
        return [AgentAction(action_type="data_pipeline", description="Designed data pipeline", parameters={"task_id": context.task_id})]
