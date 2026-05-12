"""QA Engineer agent - designs test strategy and automated validation."""

from __future__ import annotations

from typing import Any

from forge_agents.base import AgentAction, AgentRole, BaseAgent, TaskContext


class QAEngineerAgent(BaseAgent):
    @property
    def role(self) -> AgentRole:
        return AgentRole.QA_ENGINEER

    @property
    def capabilities(self) -> list[str]:
        return ["test_generation", "test_execution", "regression_testing", "acceptance_testing"]

    @property
    def system_prompt(self) -> str:
        return "You are a QA engineer. You design comprehensive test strategies and write automated tests."

    async def observe(self, context: TaskContext) -> dict[str, Any]:
        return {"task": context.task_description}

    async def think(self, context: TaskContext, observations: dict[str, Any]) -> list[dict[str, Any]]:
        return [{"step": "write_tests", "description": "Write test cases"}]

    async def act(self, context: TaskContext, plan: list[dict[str, Any]]) -> list[AgentAction]:
        return [AgentAction(action_type="test_generation", description="Generated tests", parameters={"task_id": context.task_id})]
