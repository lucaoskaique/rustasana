---
name: rustasana
description: Manage Asana tasks from the command line using the rustasana CLI tool
license: MIT
compatibility: opencode
metadata:
  category: productivity
  tool: rustasana
  version: 0.1.0
---

## What I do

I help you manage your Asana tasks directly from the terminal using the `rustasana` CLI tool. I can:

- List all your assigned tasks with due dates
- View detailed task information including notes and comments
- Mark tasks as complete
- Set or update due dates (supports natural language like "today" or "tomorrow")
- Add comments to tasks via your editor
- Open tasks in your web browser
- Download attachments from tasks
- Manage workspaces

## When to use me

Use this skill when you need to:

- Check your Asana tasks without leaving the terminal
- Update task status while working on code
- Review task details and comments
- Manage task due dates
- Coordinate code work with Asana project management

## Prerequisites

The `rustasana` CLI tool must be installed and configured:

```bash
# Check if rustasana is installed
rustasana --version

# If not configured, run:
rustasana config
```

You'll need:
- Rustasana CLI installed (`cargo install rustasana` or built from source)
- Asana Personal Access Token (get from https://app.asana.com/-/account_api)
- At least one Asana workspace

## Available commands

### List tasks
```bash
rustasana tasks           # List all tasks (uses cache)
rustasana ts              # Short alias
rustasana tasks --refresh # Force refresh from API
rustasana tasks --no-cache # Bypass cache completely
```

Output format:
```
 0 [ 2024-04-20 ] Complete project documentation
 1 [ 2024-04-21 ] Review pull requests
 2 [            ] Update README
```

Task indexes start at 0 and are used for all other commands.

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
# See all tasks
rustasana tasks

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
- **Indexes**: Task indexes are stable within the cache period but may change after refresh.
- **Configuration**: Settings stored in `~/.asana.yml`, cache in `~/.asana.cache`
- **Editor**: Set `$EDITOR` environment variable for comment editing (default: vi/notepad)
- **Browser**: Set `$BROWSER` environment variable or uses system default

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

**Want to reconfigure**
```bash
# Just run config again with new token
rustasana config
```

## Tips for AI agents

When using this skill:

1. **Always list tasks first** before operating on specific tasks by index
2. **Check task details** with `-v` flag when context is needed
3. **Use cache wisely** - refresh only when you need current data
4. **Respect indexes** - they're 0-based and can change after refresh
5. **Combine with git** - relate tasks to branches/commits for context
6. **Natural due dates** - prefer "today"/"tomorrow" over specific dates when appropriate

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
# 1. List all tasks
rustasana tasks

# 2. Review high-priority tasks
rustasana task 0 -v
rustasana task 1 -v

# 3. Set due dates
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
