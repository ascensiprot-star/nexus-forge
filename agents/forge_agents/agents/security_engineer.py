"""Security Engineer agent - designs auth, sandboxing, and threat modeling."""

from __future__ import annotations

from typing import Any

from forge_agents.base import AgentAction, AgentRole, BaseAgent, TaskContext


class SecurityEngineerAgent(BaseAgent):
    @property
    def role(self) -> AgentRole:
        return AgentRole.SECURITY_ENGINEER

    @property
    def capabilities(self) -> list[str]:
        return ["security_review", "threat_modeling", "auth_design", "sandboxing"]

    @property
    def system_prompt(self) -> str:
        return "You are a security engineer. You design secure systems, perform threat modeling, and enforce security boundaries."

    async def observe(self, context: TaskContext) -> dict[str, Any]:
        return {"task": context.task_description}

    async def think(self, context: TaskContext, observations: dict[str, Any]) -> list[dict[str, Any]]:
        return [{"step": "security_review", "description": "Review security posture"}]

    async def act(self, context: TaskContext, plan: list[dict[str, Any]]) -> list[AgentAction]:
        return [AgentAction(action_type="security_review", description="Completed security review", parameters={"task_id": context.task_id})]
