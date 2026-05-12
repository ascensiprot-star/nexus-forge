"""Base agent class for all Nexus Forge agents."""

from __future__ import annotations

import uuid
from abc import ABC, abstractmethod
from enum import Enum
from typing import Any

import structlog
from pydantic import BaseModel, Field

logger = structlog.get_logger()


class AgentStatus(str, Enum):
    IDLE = "idle"
    PREPARING = "preparing"
    THINKING = "thinking"
    ACTING = "acting"
    OBSERVING = "observing"
    REFLECTING = "reflecting"
    WAITING_APPROVAL = "waiting_approval"
    BLOCKED = "blocked"
    DONE = "done"
    ERROR = "error"


class AgentRole(str, Enum):
    AI_ENGINEER = "ai_engineer"
    SOFTWARE_ENGINEER = "software_engineer"
    DATA_ENGINEER = "data_engineer"
    BACKEND_ARCHITECT = "backend_architect"
    FRONTEND_ENGINEER = "frontend_engineer"
    DEVOPS_ENGINEER = "devops_engineer"
    SECURITY_ENGINEER = "security_engineer"
    QA_ENGINEER = "qa_engineer"
    DATABASE_ARCHITECT = "database_architect"
    PERFORMANCE_ENGINEER = "performance_engineer"
    DOCUMENTATION_ENGINEER = "documentation_engineer"
    CODE_REVIEWER = "code_reviewer"


class TaskContext(BaseModel):
    task_id: str
    task_title: str
    task_description: str
    project_id: str
    project_path: str
    memory_context: list[dict[str, Any]] = Field(default_factory=list)
    file_context: dict[str, str] = Field(default_factory=dict)
    constraints: list[str] = Field(default_factory=list)
    dependencies: list[str] = Field(default_factory=list)


class AgentAction(BaseModel):
    action_id: str = Field(default_factory=lambda: str(uuid.uuid4()))
    action_type: str
    description: str
    parameters: dict[str, Any] = Field(default_factory=dict)
    requires_approval: bool = False
    risk_level: str = "low"


class AgentOutput(BaseModel):
    task_id: str
    agent_id: str
    status: str
    actions_taken: list[AgentAction] = Field(default_factory=list)
    artifacts: list[dict[str, str]] = Field(default_factory=list)
    summary: str = ""
    tokens_used: int = 0
    duration_ms: int = 0
    errors: list[str] = Field(default_factory=list)


class BaseAgent(ABC):
    """Base class for all Nexus Forge agents.

    Each agent follows the Observe → Think → Act → Reflect loop:
    1. Observe: gather context, read task inputs
    2. Think: reason about approach, plan steps
    3. Act: execute planned actions (code gen, file ops, etc.)
    4. Reflect: evaluate output quality, decide if done
    """

    def __init__(self, agent_id: str | None = None):
        self.agent_id = agent_id or str(uuid.uuid4())
        self.status = AgentStatus.IDLE
        self.log = logger.bind(agent_id=self.agent_id, role=self.role.value)

    @property
    @abstractmethod
    def role(self) -> AgentRole:
        """The agent's role."""

    @property
    @abstractmethod
    def capabilities(self) -> list[str]:
        """List of capabilities this agent provides."""

    @property
    def system_prompt(self) -> str:
        """System prompt for the agent's LLM calls."""
        return f"You are a {self.role.value} agent in the Nexus Forge platform."

    async def execute(self, context: TaskContext) -> AgentOutput:
        """Main execution loop: observe → think → act → reflect."""
        self.status = AgentStatus.PREPARING
        self.log.info("starting task", task_id=context.task_id)

        output = AgentOutput(
            task_id=context.task_id,
            agent_id=self.agent_id,
            status="in_progress",
        )

        try:
            self.status = AgentStatus.OBSERVING
            observations = await self.observe(context)

            self.status = AgentStatus.THINKING
            plan = await self.think(context, observations)

            self.status = AgentStatus.ACTING
            actions = await self.act(context, plan)
            output.actions_taken = actions

            self.status = AgentStatus.REFLECTING
            reflection = await self.reflect(context, actions)

            output.status = "complete" if reflection.get("satisfied") else "needs_revision"
            output.summary = reflection.get("summary", "")

        except Exception as e:
            self.status = AgentStatus.ERROR
            output.status = "error"
            output.errors.append(str(e))
            self.log.error("task failed", error=str(e))

        self.status = AgentStatus.DONE
        self.log.info("task finished", status=output.status)
        return output

    @abstractmethod
    async def observe(self, context: TaskContext) -> dict[str, Any]:
        """Gather context and understand the task."""

    @abstractmethod
    async def think(self, context: TaskContext, observations: dict[str, Any]) -> list[dict[str, Any]]:
        """Plan the approach based on observations."""

    @abstractmethod
    async def act(self, context: TaskContext, plan: list[dict[str, Any]]) -> list[AgentAction]:
        """Execute the planned actions."""

    async def reflect(self, context: TaskContext, actions: list[AgentAction]) -> dict[str, Any]:
        """Evaluate output quality. Override for custom reflection."""
        return {
            "satisfied": len(actions) > 0 and all(a.action_type != "error" for a in actions),
            "summary": f"Completed {len(actions)} actions",
        }
