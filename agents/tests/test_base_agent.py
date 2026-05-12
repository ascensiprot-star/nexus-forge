"""Tests for the base agent class."""

from __future__ import annotations

from typing import Any

import pytest

from forge_agents.base import AgentAction, AgentRole, AgentStatus, BaseAgent, TaskContext


class MockAgent(BaseAgent):
    @property
    def role(self) -> AgentRole:
        return AgentRole.SOFTWARE_ENGINEER

    @property
    def capabilities(self) -> list[str]:
        return ["test"]

    async def observe(self, context: TaskContext) -> dict[str, Any]:
        return {"observed": True}

    async def think(
        self, context: TaskContext, observations: dict[str, Any]
    ) -> list[dict[str, Any]]:
        return [{"step": "test", "description": "test step"}]

    async def act(
        self, context: TaskContext, plan: list[dict[str, Any]]
    ) -> list[AgentAction]:
        return [
            AgentAction(action_type="test", description="test action")
        ]


@pytest.fixture
def agent() -> MockAgent:
    return MockAgent(agent_id="test-agent")


@pytest.fixture
def context() -> TaskContext:
    return TaskContext(
        task_id="task-1",
        task_title="Test Task",
        task_description="A test task",
        project_id="proj-1",
        project_path="/tmp/test",
    )


@pytest.mark.asyncio
async def test_agent_execution(agent: MockAgent, context: TaskContext) -> None:
    output = await agent.execute(context)
    assert output.status == "complete"
    assert output.task_id == "task-1"
    assert output.agent_id == "test-agent"
    assert len(output.actions_taken) == 1
    assert agent.status == AgentStatus.DONE


@pytest.mark.asyncio
async def test_agent_role(agent: MockAgent) -> None:
    assert agent.role == AgentRole.SOFTWARE_ENGINEER


def test_task_context_creation(context: TaskContext) -> None:
    assert context.task_id == "task-1"
    assert context.project_path == "/tmp/test"
    assert context.memory_context == []


def test_agent_action_creation() -> None:
    action = AgentAction(
        action_type="write_file",
        description="Write main.py",
        parameters={"path": "main.py"},
    )
    assert action.action_type == "write_file"
    assert action.requires_approval is False
