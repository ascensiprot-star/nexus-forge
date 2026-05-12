"""Structured prompt templates for agent reasoning loops."""

from __future__ import annotations

PLANNING_PROMPT = """Analyze the following user request and break it into concrete implementation tasks.

User Request: {intent}

Project Context:
{context}

Produce a structured plan with:
1. Architecture decisions needed
2. Implementation tasks (ordered by dependency)
3. Test requirements
4. Estimated effort per task

Format as JSON with keys: tasks, decisions, tests, total_estimated_minutes
"""

CODE_GENERATION_PROMPT = """Generate production-quality code for the following task.

Task: {task_description}

Existing Code Context:
{file_context}

Conventions:
{conventions}

Requirements:
- Follow existing code conventions
- Include error handling
- Add appropriate comments for complex logic only
- Ensure the code is immediately runnable
"""

CODE_REVIEW_PROMPT = """Review the following code changes for quality, correctness, and security.

Files Changed:
{diff}

Review Criteria:
1. Correctness: Does the code do what it should?
2. Security: Any vulnerabilities or unsafe patterns?
3. Performance: Any inefficient patterns?
4. Maintainability: Is it readable and well-structured?
5. Naming: Are names clear and consistent?

Provide specific feedback with file paths and line numbers.
Format: APPROVE, REQUEST_CHANGES, or COMMENT
"""

TEST_GENERATION_PROMPT = """Generate comprehensive tests for the following code.

Code:
{code}

Language: {language}
Test Framework: {test_framework}

Include:
- Unit tests for each public function/method
- Edge case tests
- Error handling tests
- Integration tests where appropriate
"""
