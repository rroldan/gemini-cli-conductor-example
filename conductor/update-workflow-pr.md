# Plan: Update Workflow to Include 'Ask User to Prepare Pull Request'

## Objective
Update the project's workflow and implementation plan to reflect that the user should be asked to prepare the pull request after a task is completed but before it is finalized.

## Key Files & Context
- `conductor/workflow.md`: The central source of truth for the project's task lifecycle.
- `conductor/tracks/build_core_todo_crud_api_20260321/plan.md`: The current track's implementation plan.

## Implementation Steps

### 1. Update Workflow.md
- Add a new step to the **Standard Task Workflow** section:
    - **Step 12: Ask User to Prepare Pull Request**
    - **Description:** After completing all previous steps (implementation, testing, commits, git notes, plan updates), inform the user that the task is ready and ask them to prepare the pull request.

### 2. Update Plan.md
- Add the `- [ ] Ask user to prepare pull request` sub-task back to the `Initialize Rust Project` task.
- Ensure all other main tasks in the plan also have this sub-task if they are intended to be separate PRs. (Based on common practice, each task should be a PR).

## Verification & Testing
- Read the updated `workflow.md` and `plan.md` to ensure clarity and correctness.
- The next time a task is completed, I will follow the updated workflow and ask the user to prepare the PR.
