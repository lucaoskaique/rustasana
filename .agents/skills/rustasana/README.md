# Rustasana AI Agent Skill

Universal skill definition for AI coding assistants to learn how to use the Rustasana CLI tool.

## Compatibility

This skill works with:
- ✅ **OpenCode** - Reads from `.opencode/skills/` or `.agents/skills/`
- ✅ **Claude Desktop** - Reads from `.claude/skills/` or `.agents/skills/`
- ✅ **Cursor** - Reads from `.cursor/skills/` or `.agents/skills/`
- ✅ **GitHub Copilot** - Can reference skill files
- ✅ **Windsurf** - Supports agent skills
- ✅ **Any AI CLI tool** - Standard markdown format

## Quick Start

### Option 1: Use in This Repository (Recommended)

The skill is already available! Just start your AI tool in this directory:

```bash
cd /path/to/rustasana
# Start your AI tool (opencode, cursor, etc.)
```

Then mention it:
```
Load the rustasana skill and show me my tasks
```

### Option 2: Install Globally

Make the skill available in all your projects:

```bash
# For OpenCode
mkdir -p ~/.config/opencode/skills
ln -s /path/to/rustasana/.agents/skills/rustasana ~/.config/opencode/skills/

# For Claude Desktop  
mkdir -p ~/.claude/skills
ln -s /path/to/rustasana/.agents/skills/rustasana ~/.claude/skills/

# For Cursor
mkdir -p ~/.cursor/skills
ln -s /path/to/rustasana/.agents/skills/rustasana ~/.cursor/skills/

# Or copy instead of symlink
cp -r .agents/skills/rustasana ~/.config/opencode/skills/
```

### Option 3: Project-Specific Installation

Copy to your project:

```bash
# In your project root
mkdir -p .agents/skills
cp -r /path/to/rustasana/.agents/skills/rustasana .agents/skills/

# Or create symlink
ln -s /path/to/rustasana/.agents/skills/rustasana .agents/skills/
```

## What the Skill Provides

### Complete Command Reference
- All rustasana commands with examples
- Flags and options explained
- Output format descriptions
- Short aliases documented

### Workflow Patterns
- Morning review routine
- Feature development workflow
- Sprint planning guide
- Code review process
- Daily standup helpers

### Integration Examples
- Git branch creation from tasks
- GitHub PR integration
- Shell scripting examples
- JSON parsing for automation
- Watch commands for monitoring

### Best Practices
- When to use cache vs. refresh
- How task indexes work
- Error handling strategies
- Rate limit management
- Security considerations

### Troubleshooting
- Common errors and solutions
- Configuration issues
- PATH problems
- Cache management

## Usage Examples

### With OpenCode

```bash
opencode
```

Then:
```
Use the rustasana skill to show me tasks due this week
```

The AI will:
1. Load the skill
2. Understand rustasana commands
3. Run appropriate commands
4. Parse and present results

### With Claude Desktop

In your conversation:
```
I have tasks in Asana. Can you help me manage them using the rustasana CLI?
```

Claude will discover and load the skill automatically.

### With Cursor

```
@rustasana Show me my top 3 tasks
```

Cursor can reference the skill to understand rustasana commands.

### With GitHub Copilot

In your code or terminal:
```
# Show Asana tasks
```

Copilot can suggest rustasana commands based on the skill.

## Directory Structure

```
.agents/skills/rustasana/
├── SKILL.md          # The skill definition (main file)
└── README.md         # This file
```

**Important:** 
- Directory name (`rustasana`) must match the `name` field in SKILL.md frontmatter
- SKILL.md must be in ALL CAPS
- Frontmatter must include `name` and `description`

## Skill Format

```markdown
---
name: rustasana
description: Manage Asana tasks from the command line
license: MIT
compatibility: opencode, claude, cursor, windsurf, github-copilot
metadata:
  category: productivity
  tool: rustasana
  version: 0.1.0
---

# Skill content here...
```

### Required Fields
- `name` - Must be lowercase, alphanumeric with hyphens, 1-64 chars
- `description` - Brief description, 1-1024 chars

### Optional Fields
- `license` - License type (e.g., MIT)
- `compatibility` - Which AI tools support it
- `metadata` - Custom key-value pairs

## Customization

Feel free to customize SKILL.md for your needs:

1. **Add team conventions:**
   ```markdown
   ## Our Team's Workflow
   
   We always create git branches from task names...
   ```

2. **Project-specific patterns:**
   ```markdown
   ## Project Integration
   
   In this project, we link tasks to GitHub issues...
   ```

3. **Custom commands:**
   ```markdown
   ## Custom Aliases
   
   We use these shell aliases:
   - `at` = `rustasana tasks`
   - `ad` = `rustasana done`
   ```

## Testing the Skill

### Verify Installation

```bash
# Check the file exists
ls -la .agents/skills/rustasana/SKILL.md

# View the content
cat .agents/skills/rustasana/SKILL.md | head -20

# Verify frontmatter
cat .agents/skills/rustasana/SKILL.md | grep -A 10 "^---$" | head -15
```

### Test with AI Tool

Start your AI tool and try:
```
Load the rustasana skill
```

Or:
```
What skills do you have available?
```

The AI should list or reference the rustasana skill.

## Permissions

Some AI tools allow configuring skill permissions.

### OpenCode Example

In `opencode.json`:
```json
{
  "permission": {
    "skill": {
      "rustasana": "allow"
    }
  }
}
```

Options:
- `allow` - Loads automatically when requested
- `deny` - Hidden from agent
- `ask` - Prompts user before loading

## Troubleshooting

### Skill Not Loading?

**Check these:**

1. ✅ Filename is `SKILL.md` (all caps)
2. ✅ Directory name matches frontmatter `name` field
3. ✅ Frontmatter has required fields (`name`, `description`)
4. ✅ Name follows format rules (lowercase, hyphens only)
5. ✅ File is in correct location (`.agents/skills/rustasana/`)

### Still Not Working?

**Try:**

1. **Restart your AI tool** - Skills are loaded on startup
2. **Check permissions** - May be denied in config
3. **Verify path** - Use absolute path to confirm location
4. **Check AI tool docs** - Specific tool may have different requirements

### Name Validation

Valid skill names:
- ✅ `rustasana`
- ✅ `my-tool`
- ✅ `task-manager-v2`

Invalid skill names:
- ❌ `Rustasana` (uppercase)
- ❌ `rust_asana` (underscore)
- ❌ `-rustasana` (starts with hyphen)
- ❌ `rustasana-` (ends with hyphen)
- ❌ `rust--asana` (consecutive hyphens)

## Sharing the Skill

### Include in Repository (Current)
✅ Already done! The skill is part of this repo.

Anyone who clones the repository gets the skill automatically.

### Publish Separately

Create a dedicated skills repository:

```bash
# Create skills collection
mkdir my-ai-skills
cd my-ai-skills
mkdir -p .agents/skills
cp -r /path/to/rustasana/.agents/skills/rustasana .agents/skills/

# Users can clone and link
git clone https://github.com/you/my-ai-skills
ln -s ~/my-ai-skills/.agents/skills/rustasana ~/.config/opencode/skills/
```

### Distribute with CLI

Include skill installation in rustasana installation docs:

```bash
# After installing rustasana
cargo install rustasana

# Optionally install the AI skill
mkdir -p ~/.config/opencode/skills
cp -r /path/to/skill ~/.config/opencode/skills/rustasana
```

## Benefits

### For Individual Users
- ✅ AI knows how to use rustasana correctly
- ✅ Consistent command usage across sessions
- ✅ Best practices automatically followed
- ✅ Faster task management in terminal

### For Teams
- ✅ Shared understanding of rustasana workflows
- ✅ Standardized task management patterns
- ✅ Documented integration points
- ✅ Onboarding new team members easier

### For AI Agents
- ✅ Structured instructions to follow
- ✅ Clear command reference
- ✅ Known error patterns and solutions
- ✅ Integration examples for context

## Examples of AI Interactions

### Task Review
```
User: Show me what I need to work on today using rustasana

AI: [Loads rustasana skill]
    [Runs: rustasana tasks --refresh]
    
    Here are your tasks:
    
    High Priority (due today):
    0 [ 2024-04-15 ] Fix authentication bug
    
    This Week:
    1 [ 2024-04-17 ] Update documentation
    2 [ 2024-04-18 ] Code review for PR #123
    
    Would you like details on any of these?
```

### Feature Development
```
User: I want to start working on task 0

AI: [Runs: rustasana task 0 -v]
    
    Task: Fix authentication bug
    Due: Today (2024-04-15)
    
    Notes:
    Users are getting logged out unexpectedly. 
    Check session timeout settings.
    
    Comments:
    - @john: This might be related to the Redis config
    - @sarah: Reproduced on staging
    
    I'll help you create a branch for this:
    
    [Suggests: git checkout -b fix/authentication-bug]
```

### Task Completion
```
User: I just fixed the auth issue. Update the task.

AI: [Runs: rustasana done 0]
    
    Great! Task 0 "Fix authentication bug" is marked as complete.
    
    Would you like me to:
    1. Add a comment with your commit hash?
    2. Check if there are related tasks to update?
    3. Show your next task?
```

## Advanced Usage

### Combined with Git
```
User: Create a branch for my highest priority task

AI: [Runs: rustasana tasks]
    [Parses task 0]
    [Creates: git checkout -b feature/task-name]
```

### Automated Reporting
```
User: Generate a daily standup report from my Asana tasks

AI: [Runs: rustasana tasks]
    [Filters by date]
    [Formats output]
    
    Daily Standup:
    ✅ Completed: Task A, Task B
    🚧 In Progress: Task C
    📋 Planned: Task D, Task E
```

### Sprint Planning
```
User: Help me plan this week's sprint using my Asana tasks

AI: [Runs: rustasana tasks]
    [Analyzes due dates and priorities]
    [Suggests: rustasana due X YYYY-MM-DD for each task]
    
    Suggested Sprint Plan:
    Monday: Tasks 0, 1
    Tuesday: Task 2
    ...
```

## Contributing

Improvements to this skill are welcome!

**How to contribute:**
1. Fork the rustasana repository
2. Edit `.agents/skills/rustasana/SKILL.md`
3. Test with your AI tool
4. Submit a pull request

**What to add:**
- Better workflow examples
- Team-specific patterns
- Integration with other tools
- Troubleshooting tips
- Advanced usage examples

## Resources

### Documentation
- [Rustasana README](../../README.md)
- [OpenCode Skills Guide](https://opencode.ai/docs/skills/)
- [Claude Skills Documentation](https://docs.anthropic.com/claude/docs)

### Tools
- [OpenCode](https://opencode.ai)
- [Claude Desktop](https://claude.ai)
- [Cursor](https://cursor.sh)
- [GitHub Copilot](https://github.com/features/copilot)
- [Windsurf](https://www.codeium.com/windsurf)

### Community
- GitHub Issues: Report problems
- Discussions: Share workflows
- Pull Requests: Contribute improvements

## License

MIT - Same as Rustasana

## Version

Skill version: 0.1.0  
Compatible with: rustasana v0.1.0+

Check for updates: See the rustasana repository
