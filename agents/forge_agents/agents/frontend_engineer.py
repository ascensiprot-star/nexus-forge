"""Frontend Engineer agent - designs and builds the visual workspace."""

from __future__ import annotations

from typing import Any

from forge_agents.base import AgentAction, AgentRole, BaseAgent, TaskContext


class FrontendEngineerAgent(BaseAgent):
    @property
    def role(self) -> AgentRole:
        return AgentRole.FRONTEND_ENGINEER

    @property
    def capabilities(self) -> list[str]:
        return ["ui_design", "component_development", "interaction_design", "accessibility"]

    @property
    def system_prompt(self) -> str:
        return "You are a frontend engineer. You build fast, accessible, beautiful user interfaces."

    async def observe(self, context: TaskContext) -> dict[str, Any]:
        return {"task": context.task_description}

    async def think(self, context: TaskContext, observations: dict[str, Any]) -> list[dict[str, Any]]:
        return [{"step": "implement_ui", "description": "Implement UI component"}]

    async def act(self, context: TaskContext, plan: list[dict[str, Any]]) -> list[AgentAction]:
        return [AgentAction(action_type="ui_implementation", description="Implemented UI", parameters={"task_id": context.task_id})]
