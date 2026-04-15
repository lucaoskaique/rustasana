# Quick Start Guide

This guide will help you get started with Rustasana in 5 minutes.

## Installation

### Option 1: Install Rust (if not already installed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Option 2: Build the Project

```bash
cd rustasana
cargo build --release
```

The binary will be at `target/release/rustasana`

### Option 3: Install System-wide

```bash
cd rustasana
cargo install --path .
```

This installs the `rustasana` command globally.

## First-Time Setup

### 1. Get Your Asana API Token

1. Visit: https://app.asana.com/-/account_api
2. Navigate to: **Settings** > **Apps** > **Manage Developer Apps** > **Personal Access Tokens**
3. Click **"+ Create New Personal Access Token"**
4. Give it a name (e.g., "CLI Access")
5. Copy the token (you won't be able to see it again!)

### 2. Configure the CLI

```bash
rustasana config
```

Paste your token when prompted, then select your workspace.

## Basic Usage

### List Your Tasks

```bash
rustasana tasks
```

### View a Task

```bash
rustasana task 0
```

### Complete a Task

```bash
rustasana done 0
```

### Set a Due Date

```bash
rustasana due 0 today
rustasana due 1 tomorrow
rustasana due 2 2024-12-25
```

### Add a Comment

```bash
rustasana comment 0
```

Your editor will open. Type your comment, save, and close.

### Open in Browser

```bash
rustasana browse 0
```

## Tips

1. **Use aliases**: Most commands have short versions
   - `rustasana ts` instead of `rustasana tasks`
   - `rustasana t` instead of `rustasana task`
   - `rustasana b` instead of `rustasana browse`

2. **Cache is your friend**: Tasks are cached for 5 minutes
   - Force refresh: `rustasana ts -r`
   - Skip cache: `rustasana ts -n`

3. **Set your editor**: 
   ```bash
   export EDITOR=vim  # or nano, code, emacs, etc.
   ```

4. **Get help anytime**:
   ```bash
   asana --help
   asana task --help
   ```

## Common Workflows

### Morning Routine

```bash
# Check your tasks
asana ts

# Review the first task
asana t 0 -v

# Set due date for today
rustasana due 0 today
```

### Completing Work

```bash
# List tasks
asana ts

# Add progress comment
asana cm 2

# Mark as done
rustasana done 2
```

### Planning Ahead

```bash
# List all tasks
asana ts

# Set due dates
rustasana due 3 tomorrow
rustasana due 4 2024-08-30

# Add planning notes
asana cm 3
```

## Troubleshooting

### Command not found

If `asana` command is not found after `cargo install`, add Cargo's bin directory to your PATH:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Add this to your `~/.bashrc`, `~/.zshrc`, or equivalent.

### Editor not opening

Set your EDITOR environment variable:

```bash
export EDITOR=nano
```

### Cache issues

Refresh the cache:

```bash
asana tasks --refresh
```

## Next Steps

- Read the full [README.md](README.md) for detailed documentation
- Check [CONTRIBUTING.md](CONTRIBUTING.md) if you want to contribute
- Report issues or request features on the GitHub repository

## Configuration Files

- Config: `~/.asana.yml`
- Cache: `~/.asana.cache`

You can manually edit `~/.asana.yml` if needed:

```yaml
personal_access_token: "your-token-here"
workspace: "workspace-gid"
```

## Pro Tips

1. Create shell aliases for common commands:
   ```bash
   alias at="rustasana tasks"
   alias ad="rustasana done"
   ```

2. Use with `watch` for a dashboard:
   ```bash
   watch -n 60 rustasana tasks
   ```

3. Pipe output for processing:
   ```bash
   rustasana tasks | grep "urgent"
   ```

Happy task managing!
