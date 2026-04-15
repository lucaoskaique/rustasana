---
name: rustasana
description: Manage Asana tasks from the command line using the rustasana CLI tool
license: MIT
compatibility: opencode, claude, cursor, windsurf, github-copilot, any-ai-agent
metadata:
  category: productivity
  tool: rustasana
  version: 0.1.0
  author: Rustasana Contributors
---

# Rustasana - Asana CLI Tool

A fast, efficient command-line tool for managing Asana tasks. Use this skill to interact with Asana without leaving your terminal.

## Quick Reference

**Installation Check**
```bash
rustasana --version  # Should show: rustasana 0.1.0
```

**Common Commands**
```bash
rustasana tasks              # List all tasks
rustasana task 0             # View task details
rustasana done 0             # Mark task complete
rustasana due 0 today        # Set due date
rustasana comment 0          # Add comment (opens editor)
rustasana browse 0           # Open in browser
```

## Setup Required

Before using rustasana, ensure:
1. ✅ CLI is installed: `cargo install rustasana` or built from source
2. ✅ Configured with token: `rustasana config`
3. ✅ In PATH: `which rustasana` shows the binary location

Configuration is stored in `~/.asana.yml` and requires an Asana Personal Access Token from https://app.asana.com/-/account_api

## Commands Reference

### List Tasks
```bash
rustasana tasks              # Use cached list (5 min cache)
rustasana ts                 # Short alias
rustasana tasks --refresh    # Force API refresh
rustasana tasks --no-cache   # Bypass cache entirely
```

**Output format:**
```
 0 [ 2024-04-20 ] Complete project documentation
 1 [ 2024-04-21 ] Review pull requests
 2 [            ] Update README
```
- First column: Task index (0-based)
- Second column: Due date (YYYY-MM-DD or empty)
- Third column: Task name

### View Task Details
```bash
rustasana task <index>       # Basic task info
rustasana t <index>          # Short alias
rustasana task <index> -v    # Verbose: include comments & history
rustasana task <index> --json # JSON output for parsing
```

### Complete Task
```bash
rustasana done <index>       # Mark as complete
```

Example:
```bash
rustasana done 0             # Completes task at index 0
```

### Set Due Date
```bash
rustasana due <index> <date>
```

Date formats:
- `today` - Sets to current date
- `tomorrow` - Sets to next day
- `YYYY-MM-DD` - Specific date (e.g., 2024-12-31)

Examples:
```bash
rustasana due 0 today
rustasana due 1 tomorrow
rustasana due 2 2024-12-25
```

### Add Comment
```bash
rustasana comment <index>    # Opens $EDITOR
rustasana cm <index>         # Short alias
```

**Note:** This opens your default editor (set via `$EDITOR` environment variable). Lines starting with `#` are ignored as comments.

### Open in Browser
```bash
rustasana browse <index>     # Opens task in default browser
rustasana b <index>          # Short alias
```

### Download Attachment
```bash
rustasana download <task_index> <attachment_index>
rustasana dl <task_index> <attachment_index> -o <output_path>
```

First, view task to see attachments:
```bash
rustasana task 0
# Shows:
# Attachments:
#   [0] document.pdf (asana)
#   [1] image.png (asana)

rustasana download 0 0                     # Downloads document.pdf
rustasana dl 0 1 -o ~/Downloads/img.png    # Downloads with custom name
```

### List Workspaces
```bash
rustasana workspaces         # List all workspaces
rustasana w                  # Short alias
```

Shows current workspace with `*` indicator.

## Important Concepts

### Task Indexes
- Tasks are numbered starting from **0**
- Indexes are shown when you run `rustasana tasks`
- Indexes are stable during cache period (5 minutes)
- After cache refresh, indexes may change if tasks are added/removed/reordered
- **Always list tasks first** before using indexes

### Caching
- Task list is cached for **5 minutes** by default
- Cache location: `~/.asana.cache`
- Use `--refresh` to force update
- Use `--no-cache` to bypass completely
- Cache improves speed and reduces API calls

### Configuration
- Config file: `~/.asana.yml`
- Contains: Personal Access Token and Workspace ID
- Run `rustasana config` to reconfigure anytime

## Workflow Patterns

### Morning Review
```bash
# 1. Get fresh task list
rustasana tasks --refresh

# 2. Review top priority tasks
rustasana task 0 -v
rustasana task 1 -v

# 3. Plan your day based on due dates
```

### Feature Development
```bash
# 1. Check assigned tasks
rustasana tasks

# 2. View task details
rustasana task 0 -v

# 3. Work on the feature
# (create branch, code, test)

# 4. Mark complete when done
rustasana done 0
```

### Sprint Planning
```bash
# 1. List all current tasks
rustasana tasks --refresh

# 2. Set due dates for the sprint
rustasana due 0 2024-04-22  # Sprint day 1
rustasana due 1 2024-04-23  # Sprint day 2
rustasana due 2 2024-04-24  # Sprint day 3

# 3. Review task details
rustasana task 0
rustasana task 1
rustasana task 2
```

### Daily Standup
```bash
# Quick overview of tasks
rustasana tasks | head -5

# Check what was completed (look for completed_at field)
rustasana task 0 --json | grep completed
```

### Code Review
```bash
# 1. Find review tasks
rustasana tasks | grep -i review

# 2. View details
rustasana task <index> -v

# 3. After review, add comment
rustasana comment <index>
# (In editor: "Reviewed - approved with minor suggestions")

# 4. Mark complete
rustasana done <index>
```

## Integration Examples

### With Git
```bash
# Create branch from task
TASK_NAME=$(rustasana task 0 --json | jq -r '.name' | tr ' ' '-' | tr '[:upper:]' '[:lower:]')
git checkout -b "feature/$TASK_NAME"
```

### With GitHub CLI
```bash
# Create PR with task context
TASK_DETAILS=$(rustasana task 0)
gh pr create --title "Fixes task from Asana" --body "$TASK_DETAILS"
```

### In Shell Scripts
```bash
#!/bin/bash
# Daily task report

echo "=== Today's Tasks ==="
rustasana tasks --refresh | grep $(date +%Y-%m-%d)

echo -e "\n=== Overdue Tasks ==="
rustasana tasks | awk -v today=$(date +%Y-%m-%d) '$2 < today'
```

### With Watch
```bash
# Live dashboard (refreshes every 60 seconds)
watch -n 60 rustasana tasks
```

## Best Practices for AI Agents

### 1. Always List First
Before operating on tasks, run `rustasana tasks` to get current indexes:
```bash
# ✅ Good
rustasana tasks
rustasana task 0

# ❌ Bad - index might be wrong
rustasana task 0  # Without listing first
```

### 2. Use Verbose When Context Needed
For detailed operations, use `-v` flag:
```bash
rustasana task 0 -v  # Shows notes, comments, history
```

### 3. Respect Cache Timing
- Use cache for quick checks
- Use `--refresh` when accuracy is critical
- Don't over-refresh (rate limits)

### 4. Parse JSON for Automation
For scripting, use `--json` output:
```bash
rustasana task 0 --json | jq '.due_on'
```

### 5. Combine with Other Tools
Rustasana works well with:
- `grep` for filtering
- `awk` for text processing
- `jq` for JSON parsing
- `watch` for monitoring
- Git for branch management

### 6. Handle Errors Gracefully
Check exit codes and output:
```bash
if rustasana tasks > /dev/null 2>&1; then
    echo "Tasks loaded successfully"
else
    echo "Error: Check configuration with 'rustasana config'"
fi
```

## Troubleshooting

### "Config file isn't set"
**Solution:**
```bash
rustasana config
# Follow prompts to enter token and select workspace
```

### "Command not found: rustasana"
**Solution:**
```bash
# Check if installed
which rustasana

# If not found, ensure ~/.cargo/bin is in PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Add to shell profile for persistence
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc  # or ~/.bashrc
source ~/.zshrc
```

### "Task not found at index X"
**Solution:**
```bash
# Cache might be stale, refresh
rustasana tasks --refresh

# Then try again with new indexes
```

### "Error: Failed to parse tasks"
**Solution:**
```bash
# Reinstall to get latest fixes
cd /path/to/rustasana
cargo install --path . --force
```

### Want to Switch Workspaces
**Solution:**
```bash
# Reconfigure to select different workspace
rustasana config
```

### Cache Issues
**Solution:**
```bash
# Clear cache manually
rm ~/.asana.cache

# Or use no-cache flag
rustasana tasks --no-cache
```

## Command Aliases Reference

| Full Command | Short Alias |
|-------------|-------------|
| `rustasana tasks` | `rustasana ts` |
| `rustasana task` | `rustasana t` |
| `rustasana comment` | `rustasana cm` |
| `rustasana browse` | `rustasana b` |
| `rustasana download` | `rustasana dl` |
| `rustasana workspaces` | `rustasana w` |
| `rustasana config` | `rustasana c` |

## Environment Variables

- `$EDITOR` - Editor for comments (default: vi on Unix, notepad on Windows)
- `$BROWSER` - Browser for browse command (auto-detected if not set)
- `$HOME` - Used for config/cache file locations

## Files & Locations

- Config: `~/.asana.yml` (YAML format)
- Cache: `~/.asana.cache` (text format)
- Binary: `~/.cargo/bin/rustasana` (or system location)

## Exit Codes

- `0` - Success
- `1` - Error (check error message for details)

## Rate Limits

Asana API has rate limits. If you hit them:
- Use cache instead of `--no-cache`
- Avoid rapid consecutive calls
- Use `--refresh` sparingly

## Security Notes

- Personal Access Token stored in `~/.asana.yml`
- File permissions should be `600` (read/write for user only)
- Never commit `.asana.yml` to version control
- Token can be regenerated at https://app.asana.com/-/account_api

## Advanced Usage

### Custom Task Filtering
```bash
# Tasks due this week
rustasana tasks | grep "$(date +%Y-%m-%d)"

# Tasks with specific keyword
rustasana tasks | grep -i "urgent"

# Count total tasks
rustasana tasks | wc -l
```

### Batch Operations
```bash
# Mark multiple tasks complete
for i in 0 1 2; do
    rustasana done $i
done
```

### JSON Processing
```bash
# Extract specific fields
rustasana task 0 --json | jq '{name, due_on, completed}'

# Get all task names
rustasana tasks --no-cache | awk '{$1=$2=""; print $0}' | sed 's/^ *//'
```

## Getting Help

```bash
rustasana --help              # General help
rustasana tasks --help        # Command-specific help
rustasana --version           # Show version
```

## Related Documentation

- Full README: See repository root
- Configuration Guide: Check QUICKSTART.md
- API Reference: https://developers.asana.com/docs

## Version

This skill is for **rustasana v0.1.0**

Check your version: `rustasana --version`

## When to Use This Tool

✅ **Use rustasana when you need to:**
- Check task assignments without opening browser
- Update task status while coding
- Integrate task management into development workflow
- Automate task operations via scripts
- Quick task lookups during development

❌ **Don't use rustasana for:**
- Creating new tasks (not yet implemented)
- Complex project management (use Asana web UI)
- Team collaboration features (use Asana web UI)
- Custom field management (limited support)

## Skill Update Policy

This skill file is updated with each rustasana release. Check the version in metadata.

Last updated: 2024-04-15 for rustasana v0.1.0
