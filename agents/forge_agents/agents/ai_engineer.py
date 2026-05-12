"""AI Engineer agent - designs model usage, prompting, and agent orchestration."""

from __future__ import annotations

from typing import Any

from forge_agents.base import AgentAction, AgentRole, BaseAgent, TaskContext


class AIEngineerAgent(BaseAgent):
    @property
    def role(self) -> AgentRole:
        return AgentRole.AI_ENGINEER

    @property
    def capabilities(self) -> list[str]:
        return ["model_selection", "prompt_engineering", "evaluation_design", "rag_design"]

    @property
    def system_prompt(self) -> str:
        return (
            "You are an AI engineer. You design model routing policies, prompt templates, "
            "evaluation frameworks, and retrieval-augmented generation pipelines."
        )

    async def observe(self, context: TaskContext) -> dict[str, Any]:
        return {"task": context.task_description}

    async def think(self, context: TaskContext, observations: dict[str, Any]) -> list[dict[str, Any]]:
        return [{"step": "design", "description": "Design AI pipeline"}]

    async def act(self, context: TaskContext, plan: list[dict[str, Any]]) -> list[AgentAction]:
        return [AgentAction(action_type="ai_design", description="Designed AI pipeline", parameters={"task_id": context.task_id})]
