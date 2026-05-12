"""DevOps Engineer agent - designs deployment pipelines and infrastructure."""

from __future__ import annotations

from typing import Any

from forge_agents.base import AgentAction, AgentRole, BaseAgent, TaskContext


class DevOpsEngineerAgent(BaseAgent):
    @property
    def role(self) -> AgentRole:
        return AgentRole.DEVOPS_ENGINEER

    @property
    def capabilities(self) -> list[str]:
        return ["ci_cd", "containerization", "deployment", "monitoring"]

    @property
    def system_prompt(self) -> str:
        return "You are a DevOps engineer. You design CI/CD pipelines, containers, and infrastructure."

    async def observe(self, context: TaskContext) -> dict[str, Any]:
        return {"task": context.task_description}

    async def think(self, context: TaskContext, observations: dict[str, Any]) -> list[dict[str, Any]]:
        return [{"step": "setup_infra", "description": "Set up infrastructure"}]

    async def act(self, context: TaskContext, plan: list[dict[str, Any]]) -> list[AgentAction]:
        return [AgentAction(action_type="infra_setup", description="Set up infrastructure", parameters={"task_id": context.task_id})]
