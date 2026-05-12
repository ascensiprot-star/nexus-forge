"""Backend Architect agent - designs server architecture and service boundaries."""

from __future__ import annotations

from typing import Any

from forge_agents.base import AgentAction, AgentRole, BaseAgent, TaskContext


class BackendArchitectAgent(BaseAgent):
    @property
    def role(self) -> AgentRole:
        return AgentRole.BACKEND_ARCHITECT

    @property
    def capabilities(self) -> list[str]:
        return [
            "architecture_design",
            "service_decomposition",
            "api_design",
            "data_modeling",
            "scalability_planning",
        ]

    @property
    def system_prompt(self) -> str:
        return (
            "You are a backend architect. You design scalable, maintainable system "
            "architectures. You define service boundaries, APIs, and data flows. "
            "You make technology choices based on constraints and requirements."
        )

    async def observe(self, context: TaskContext) -> dict[str, Any]:
        return {"requirements": context.task_description, "constraints": context.constraints}

    async def think(
        self, context: TaskContext, observations: dict[str, Any]
    ) -> list[dict[str, Any]]:
        return [
            {"step": "analyze_requirements", "description": "Analyze system requirements"},
            {"step": "design_architecture", "description": "Design system architecture"},
            {"step": "define_interfaces", "description": "Define service interfaces"},
        ]

    async def act(
        self, context: TaskContext, plan: list[dict[str, Any]]
    ) -> list[AgentAction]:
        return [
            AgentAction(
                action_type="architecture_design",
                description="Designed system architecture",
                parameters={"task_id": context.task_id},
            )
        ]
