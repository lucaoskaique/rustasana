---
name: rustasana
description: Manage Asana tasks from the command line using the rustasana CLI tool
license: MIT
compatibility: opencode
metadata:
  category: productivity
  tool: rustasana
  version: 0.5.0
---

## What I do

I help you manage your Asana tasks directly from the terminal using the `rustasana` CLI tool. I can:

- List all your assigned tasks with due dates
- Filter tasks by assignee (all tasks, unassigned, or specific user)
- View detailed task information including notes and comments
- Mark tasks as complete
- Assign tasks to users or unassign them
- Set or update due dates (supports natural language like "today" or "tomorrow")
- Add comments to tasks via your editor
- Open tasks in your web browser
- Download attachments from tasks
- Manage workspaces

## When to use me

Use this skill when you need to:

- Check your Asana tasks without leaving the terminal
- Update task status while working on code
- Assign tasks to team members or yourself
- Review task details and comments
- Manage task due dates
- Coordinate code work with Asana project management

## Prerequisites

The `rustasana` CLI tool must be installed and configured:

```bash
# Install from crates.io (recommended)
cargo install rustasana

# Check if rustasana is installed
rustasana --version

# If not configured, run:
rustasana config
```

You'll need:
- Rustasana CLI installed from crates.io: https://crates.io/crates/rustasana
- Asana Personal Access Token (get from https://app.asana.com/-/account_api)
- At least one Asana workspace

## Available commands

### List projects
```bash
rustasana projects        # List all projects in workspace
rustasana p               # Short alias
```

Output format:
```
Projects:
  1208684367073999 QA
  1208704905694979 Product Roadmap
  1210197328049310 Pixel Pioneers
  1211411822893284 S3
```

### List tasks
```bash
rustasana tasks                 # List your assigned tasks (uses cache)
rustasana ts                    # Short alias
rustasana tasks --refresh       # Force refresh from API
rustasana tasks --no-cache      # Bypass cache completely

# Filter by project (shows ALL tasks in project)
rustasana tasks --project 1210197328049310  # By project GID
rustasana tasks -p 1210197328049310         # Short flag

# Filter by assignee
rustasana tasks --assignee 1234  # List tasks for specific user (by GID)

# Combine flags
rustasana tasks -p 1210197328049310 --refresh  # Refresh project tasks
rustasana tasks --assignee 1234 --no-cache     # Fresh assignee tasks
```

Output format (default - your tasks):
```
 0 [ 2024-04-20 ] Complete project documentation                  [@john_doe]
 1 [ 2024-04-21 ] Review pull requests                            [@john_doe]
 2 [            ] Update README                                    [@john_doe]
```

Output format (with --project - all tasks in project):
```
 0 [ 2024-04-20 ] Complete project documentation                  [@john_doe]
 1 [ 2024-04-21 ] Review pull requests                            [@jane_smith]
 2 [            ] Update README                                    [unassigned]
 3 [ 2024-04-22 ] Fix bug in auth                                 [unassigned]
```

Output format (with --assignee - specific user's tasks):
```
 0 [ 2024-04-20 ] Complete documentation                          [@jane_smith]
 1 [ 2024-04-21 ] Review pull requests                            [@jane_smith]
```

Task indexes start at 0 and are used for all other commands.
Note: Indexes are only valid within the same cache context (e.g., project tasks vs. your tasks).

### View task details
```bash
rustasana task 0          # View task at index 0
rustasana t 0             # Short alias
rustasana task 0 -v       # Include comments and history
rustasana task 0 --json   # Output as JSON
```

### Complete a task
```bash
rustasana done 0          # Mark task 0 as complete
```

### Assign task
```bash
rustasana assign 0 me                   # Assign task 0 to yourself
rustasana assign 1 1234567890123456     # Assign task 1 to user by GID
rustasana assign 2 null                 # Unassign task 2
rustasana assign 2 unassigned           # Alternative unassign syntax
```

Note: To get a user's GID, view a task in JSON format with `rustasana task <index> --json` and look at the assignee field.

### Set due date
```bash
rustasana due 0 today           # Set to today
rustasana due 1 tomorrow        # Set to tomorrow
rustasana due 2 2024-12-31      # Set specific date (YYYY-MM-DD)
```

### Add comment
```bash
rustasana comment 0       # Opens your $EDITOR to write comment
rustasana cm 0            # Short alias
```

Note: This opens an editor. Lines starting with `#` are ignored.

### Open in browser
```bash
rustasana browse 0        # Open task 0 in default browser
rustasana b 0             # Short alias
```

### Download attachment
```bash
rustasana download 0 0                    # Download attachment 0 from task 0
rustasana dl 0 0 -o ~/Downloads/file.pdf  # With custom output path
```

### List workspaces
```bash
rustasana workspaces      # List all workspaces
rustasana w               # Short alias
```

## Workflow examples

### Check what to work on
```bash
# See your assigned tasks
rustasana tasks

# Find a project to work on
rustasana projects

# See all tasks in a specific project
rustasana tasks -p 1210197328049310

# Find available work (unassigned tasks in project)
rustasana tasks -p 1210197328049310 | grep unassigned

# View first task details
rustasana task 0 -v
```

### Complete and update
```bash
# Mark task as done
rustasana done 0

# Set next task due tomorrow
rustasana due 1 tomorrow
```

### Review and comment
```bash
# View task with full history
rustasana task 2 -v

# Add a comment
rustasana comment 2
```

## Important notes

- **Cache**: Task list is cached for 5 minutes. Use `--refresh` to update.
- **Filter caches**: Different cache files for different filters:
  - `~/.asana.cache` - Your assigned tasks (default)
  - `~/.asana.cache.project.<project_gid>` - All tasks from a specific project
  - `~/.asana.cache.<user_gid>` - Tasks for a specific user
- **Project context is critical**: When working with project tasks, ALL commands must use the same `--project` flag
  - ✅ CORRECT: `rustasana tasks -p 123` then `rustasana done 0 -p 123`
  - ❌ WRONG: `rustasana tasks -p 123` then `rustasana done 0` (uses wrong cache!)
- **Context flags**: Most commands accept `--project <GID>` or `--assignee <GID>` to specify cache context
- **Indexes**: Task indexes are stable within the cache period but may change after refresh or when switching contexts
- **Configuration**: Settings stored in `~/.asana.yml`
- **Editor**: Set `$EDITOR` environment variable for comment editing (default: vi/notepad)
- **Browser**: Set `$BROWSER` environment variable or uses system default
- **Assignee display**: Tasks show assignee names in brackets (e.g., [@username] or [unassigned])
- **Pagination**: Large projects (1000+ tasks) are automatically paginated and fetched completely

## Common patterns

### Morning routine
```bash
# Refresh and review tasks
rustasana tasks --refresh
rustasana task 0 -v
```

### After completing work
```bash
# Mark done and move to next
rustasana done 0
rustasana task 1
```

### Planning ahead
```bash
# Set due dates for the week
rustasana due 0 today
rustasana due 1 tomorrow
rustasana due 2 2024-04-25
```

### Quick status check
```bash
# See what's urgent (tasks are sorted by due date)
rustasana ts | head -5
```

## Troubleshooting

If you encounter issues:

**"Config file isn't set"**
```bash
rustasana config
```

**"Command not found"**
```bash
# Check installation
which rustasana

# If not found, ensure ~/.cargo/bin is in PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

**"Task not found at index X"**
```bash
# Cache may be stale, refresh
rustasana tasks --refresh
```

**"Project not found"**
```bash
# Make sure you're using a valid project GID
# List all projects first
rustasana projects
```

**"Assignee not found"**
```bash
# Ensure you're using a valid user GID (not email or name)
# Find GID by viewing a task in JSON format
rustasana task 0 --json | grep -A 2 '"assignee"'
```

**"Task not found at index X"**
```bash
# Cache may be stale, or you're using a different filter
# Refresh with the same filter you used to list tasks
rustasana tasks --refresh              # For your tasks
rustasana tasks -p <project_gid> --refresh  # For project tasks
```

**Want to reconfigure**
```bash
# Just run config again with new token
rustasana config
```

## Tips for AI agents

When using this skill:

1. **List projects first** to find the right project GID before fetching tasks
2. **Use project filtering** - `-p <project_gid>` to see entire project backlogs
3. **Always list tasks first** before operating on specific tasks by index
4. **Match your context** - if you listed with `-p`, viewing tasks requires that same cache
5. **Check task details** with `-v` flag when context is needed
6. **Use cache wisely** - refresh only when you need current data
7. **Respect indexes** - they're 0-based and only valid within the same cache context
8. **Combine with git** - relate tasks to branches/commits for context
9. **Natural due dates** - prefer "today"/"tomorrow" over specific dates when appropriate
10. **Large projects** - pagination is automatic, no need to worry about task limits

## Example workflows for agents

### Feature development
```bash
# 1. Check tasks
rustasana tasks

# 2. View task details
rustasana task 0 -v

# 3. Create feature branch based on task
# (use task name from output)

# 4. When done, mark complete
rustasana done 0
```

### Sprint planning
```bash
# 1. Find the sprint project
rustasana projects | grep "Sprint"

# 2. List all tasks in the sprint project
rustasana tasks -p 1210197328049310

# 3. Find unassigned work
rustasana tasks -p 1210197328049310 | grep unassigned

# 4. Review high-priority tasks
rustasana task 0 -v
rustasana task 1 -v

# 5. Set due dates
rustasana due 0 today
rustasana due 1 tomorrow
```

### Code review
```bash
# 1. Check related tasks
rustasana tasks | grep -i "review"

# 2. Add comment after review
rustasana comment <index>
# (In editor: add review notes)

# 3. Mark complete
rustasana done <index>
```

## Integration tips

Rustasana works well with:
- **Git**: Use task names for branch naming
- **GitHub**: Reference task IDs in commits/PRs
- **Shell scripts**: Pipe output to grep, awk, etc.
- **Watch**: Monitor tasks with `watch -n 60 rustasana tasks`
- **Cron**: Automate task reporting

## Related commands

```bash
rustasana --help          # Show all commands
rustasana --version       # Show version
rustasana <cmd> --help    # Help for specific command
```

## Links

- Documentation: See README.md in the rustasana repository
- Issues: Report bugs via GitHub issues
- Configuration: `~/.asana.yml` (YAML format)
- Cache: `~/.asana.cache` (text format, auto-managed)
