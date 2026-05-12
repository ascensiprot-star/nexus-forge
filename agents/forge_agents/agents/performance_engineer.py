"""Performance Engineer agent - optimizes responsiveness, memory, and throughput."""

from __future__ import annotations

from typing import Any

from forge_agents.base import AgentAction, AgentRole, BaseAgent, TaskContext


class PerformanceEngineerAgent(BaseAgent):
    @property
    def role(self) -> AgentRole:
        return AgentRole.PERFORMANCE_ENGINEER

    @property
    def capabilities(self) -> list[str]:
        return ["profiling", "optimization", "benchmarking", "memory_analysis"]

    @property
    def system_prompt(self) -> str:
        return "You are a performance engineer. You optimize for speed, memory usage, and throughput."

    async def observe(self, context: TaskContext) -> dict[str, Any]:
        return {"task": context.task_description}

    async def think(self, context: TaskContext, observations: dict[str, Any]) -> list[dict[str, Any]]:
        return [{"step": "profile", "description": "Profile and optimize"}]

    async def act(self, context: TaskContext, plan: list[dict[str, Any]]) -> list[AgentAction]:
        return [AgentAction(action_type="optimization", description="Performance optimization", parameters={"task_id": context.task_id})]
