# Rustasana OpenCode Skill

This directory contains an OpenCode skill definition that teaches AI agents how to use the Rustasana CLI tool.

## What is a Skill?

Skills are structured instruction sets that AI coding agents (like OpenCode, Cursor, Windsurf, etc.) can load on-demand to learn how to use specific tools or follow specific workflows.

## How to Use This Skill

### For OpenCode Users

The skill is already available in this repository! When you use OpenCode in this project, it will automatically discover the skill.

**To use it:**

1. Start OpenCode in any directory within this repository:
   ```bash
   cd /path/to/rustasana
   opencode
   ```

2. Ask OpenCode to use the skill:
   ```
   Load the rustasana skill
   ```
   
   Or just mention it naturally:
   ```
   Show me my Asana tasks using rustasana
   ```

3. OpenCode will load the skill and know how to:
   - List your tasks
   - View task details
   - Mark tasks complete
   - Set due dates
   - Add comments
   - And more!

### For Global Installation

To make the skill available in all your projects:

```bash
# Copy to global config
mkdir -p ~/.config/opencode/skills/rustasana
cp .opencode/skills/rustasana/SKILL.md ~/.config/opencode/skills/rustasana/

# Or create a symlink
ln -s /path/to/rustasana/.opencode/skills/rustasana ~/.config/opencode/skills/
```

Now you can use rustasana in any OpenCode session!

### For Other AI Tools

Many AI coding tools support skills in similar formats. Check if your tool supports:

- `.opencode/skills/` directory structure
- `.claude/skills/` (Claude Desktop, Cursor)
- `.agents/skills/` (generic agent tools)

You can copy or symlink the skill directory:

```bash
# For Claude/Cursor
mkdir -p .claude/skills/rustasana
cp .opencode/skills/rustasana/SKILL.md .claude/skills/rustasana/

# For generic agents
mkdir -p .agents/skills/rustasana
cp .opencode/skills/rustasana/SKILL.md .agents/skills/rustasana/
```

## What the Skill Teaches

The skill provides comprehensive instructions on:

### Commands
- `rustasana tasks` - List all tasks
- `rustasana task <index>` - View task details
- `rustasana done <index>` - Complete a task
- `rustasana due <index> <date>` - Set due date
- `rustasana comment <index>` - Add comment
- `rustasana browse <index>` - Open in browser
- `rustasana download <task> <attachment>` - Download attachment
- `rustasana workspaces` - List workspaces

### Workflows
- Morning task review
- Feature development with tasks
- Sprint planning
- Code review workflows

### Best Practices
- When to refresh cache
- How to handle task indexes
- Integration with git/GitHub
- Troubleshooting common issues

## Example Usage

Once the skill is loaded, you can have conversations like:

```
You: Show me my Asana tasks for this week

AI: [Loads rustasana skill]
    [Runs: rustasana tasks]
    Here are your tasks:
    0 [ 2024-04-20 ] Complete project documentation
    1 [ 2024-04-21 ] Review pull requests
    2 [ 2024-04-25 ] Update README
    
    The first two are due this week.

You: Show me details of the first task

AI: [Runs: rustasana task 0 -v]
    [Shows detailed task information with comments]

You: I just finished that. Mark it as complete.

AI: [Runs: rustasana done 0]
    Task marked as completed!
```

## Customization

You can customize the skill by editing `SKILL.md`:

1. Add project-specific workflows
2. Include team conventions
3. Add custom command combinations
4. Document integration with other tools

## Permissions

Control skill access in `opencode.json`:

```json
{
  "permission": {
    "skill": {
      "rustasana": "allow"  // or "deny" or "ask"
    }
  }
}
```

Options:
- `allow` - Skill loads automatically
- `deny` - Skill is hidden from agent
- `ask` - User is prompted before loading

## Directory Structure

```
.opencode/skills/rustasana/
└── SKILL.md          # The skill definition
```

The directory name must match the skill name in the frontmatter!

## Skill Format

Skills use markdown with YAML frontmatter:

```yaml
---
name: rustasana                    # Must match directory name
description: Short description     # Shown to agent
license: MIT                       # Optional
compatibility: opencode            # Which tools support it
metadata:                          # Optional key-value pairs
  category: productivity
  tool: rustasana
---

## Skill content here...
```

## Validation Rules

Skill names must:
- Be 1-64 characters
- Use lowercase letters, numbers, and single hyphens
- Not start or end with hyphen
- Not have consecutive hyphens
- Match the containing directory name

Valid: `rustasana`, `my-tool`, `task-manager-v2`  
Invalid: `Rustasana`, `-tool`, `my--tool`, `my_tool`

## Troubleshooting

### Skill not loading?

1. **Check filename**: Must be `SKILL.md` (all caps)
2. **Check frontmatter**: Must have `name` and `description`
3. **Check name format**: Must follow validation rules
4. **Check directory name**: Must match the `name` field
5. **Check permissions**: May be denied in config

### How to verify:

```bash
# In OpenCode, list available skills
# The agent will show available skills when appropriate

# Or check the files directly
ls -la .opencode/skills/rustasana/
cat .opencode/skills/rustasana/SKILL.md
```

## Sharing the Skill

Want to share this skill with others?

### Option 1: Include in your repository
Already done! Anyone cloning this repo gets the skill.

### Option 2: Publish separately
You could extract the skill to a separate repository:

```bash
# Create a skills repo
mkdir rustasana-skills
cd rustasana-skills
cp -r /path/to/rustasana/.opencode/skills/rustasana .opencode/skills/

# Users can then clone and symlink
git clone https://github.com/you/rustasana-skills ~/.config/opencode/skills/rustasana
```

### Option 3: Package with the CLI
When distributing rustasana, include instructions to copy the skill.

## Benefits of Using Skills

1. **Consistency**: AI agents use rustasana the same way every time
2. **Discoverability**: Agents know when to suggest using rustasana
3. **Best practices**: Encoded common patterns and workflows
4. **Context**: Agents understand task indexes, caching, etc.
5. **Reusable**: One skill definition works across all projects

## Next Steps

1. **Try it**: Start OpenCode and ask it to use rustasana
2. **Customize**: Edit SKILL.md to add your team's workflows
3. **Share**: Commit the skill to your repository
4. **Extend**: Create additional skills for related tools

## Resources

- [OpenCode Skills Documentation](https://opencode.ai/docs/skills/)
- [Rustasana README](../../README.md)
- [OpenCode Configuration](https://opencode.ai/docs/config/)

## Contributing

Improvements to this skill are welcome! If you find better ways to explain rustasana usage or have additional workflow examples, please submit a PR.

## License

MIT - Same as Rustasana
