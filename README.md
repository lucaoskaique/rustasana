# Rustasana

A blazingly fast command-line interface for [Asana](https://asana.com/) written in Rust. This is a Rust implementation inspired by [thash/asana](https://github.com/thash/asana).

## Features

- 🚀 Fast and efficient CLI for Asana
- 💾 Local caching for quick task listing
- 📁 Project-based task filtering with context-aware commands
- ✅ Filter incomplete tasks (default) or include completed tasks with `--all`
- 👥 Filter tasks by assignee (all, unassigned, or specific user)
- 👤 Assign tasks to users or yourself
- 📝 Add comments via your favorite editor
- 🌐 Open tasks directly in your browser
- ✅ Mark tasks as complete
- 📅 Set due dates with natural language (today, tomorrow)
- 🔍 View task details with verbose mode

## Installation

### From crates.io (Recommended)

```bash
cargo install rustasana
```

### Prerequisites

- Rust 1.70 or higher ([Install Rust](https://www.rust-lang.org/tools/install))

### Build from source

```bash
cd rustasana
cargo build --release
```

The binary will be available at `target/release/rustasana`.

### Install globally from source

```bash
cargo install --path .
```

## Usage

### Initial Configuration

First, configure your Asana credentials:

```bash
$ rustasana config
visit: http://app.asana.com/-/account_api
  Settings > Apps > Manage Developer Apps > Personal Access Tokens
  + Create New Personal Access Token

paste your Personal Access Token: _
```

1. Visit the URL shown
2. Navigate to: Settings > Apps > Manage Developer Apps > Personal Access Tokens
3. Click "+ Create New Personal Access Token"
4. Copy the token and paste it into the terminal
5. Select your workspace from the list

Your configuration will be saved to `~/.asana.yml`.

### Commands

#### List Projects

```bash
# List all projects in your workspace
$ rustasana projects

# Or use the short alias
$ rustasana p
```

Output:
```
Projects:
  1208684367073999 QA
  1208704905694979 Product Roadmap
  1210197328049310 Pixel Pioneers
  1211411822893284 S3
```

#### List Tasks

```bash
# List your assigned tasks (default - incomplete only)
$ rustasana tasks

# Or use the short alias
$ rustasana ts

# Include completed tasks
$ rustasana ts --all
$ rustasana ts -a

# List all tasks from a specific project (by project GID - incomplete only by default)
$ rustasana ts --project 1210197328049310
$ rustasana ts -p 1210197328049310

# List ALL tasks (including completed) from a project
$ rustasana ts -p 1210197328049310 --all

# List tasks for a specific user (by user GID)
$ rustasana ts --assignee 1234567890123456

# Bypass cache
$ rustasana ts --no-cache

# Refresh cache
$ rustasana ts --refresh

# Combine flags
$ rustasana ts -p 1210197328049310 --refresh
$ rustasana ts --assignee 1234567890123456 --no-cache
$ rustasana ts -p 1210197328049310 --all --no-cache
```

Output (default view - your tasks):
```
 0 [ 2024-08-13 ] Write README                                       [@john_doe]
 1 [ 2024-08-18 ] Buy gift for coworkers                             [@john_doe]
 2 [ 2024-08-29 ] Read "Unweaving the Rainbow"                       [@john_doe]
 3 [            ] haircut                                             [@john_doe]
```

Output (with --project flag - all tasks in project):
```
 0 [ 2024-08-13 ] Complete documentation                             [@john_doe]
 1 [ 2024-08-21 ] Review pull requests                               [@jane_smith]
 2 [            ] Update README                                       [unassigned]
 3 [ 2024-08-22 ] Fix bug in auth                                    [unassigned]
 4 [            ] Refactor database layer                            [@john_doe]
```

Output (with --assignee flag - specific user's tasks):
```
 0 [ 2024-08-13 ] Complete documentation                             [@jane_smith]
 1 [ 2024-08-21 ] Review pull requests                               [@jane_smith]
```

### Working with Project Context

All task index commands support `--project` and `--assignee` flags to specify which cache context to use. This is essential when working with tasks from specific projects.

**Important**: The task index you use must match the context where you listed the tasks.

```bash
# List tasks from a project
$ rustasana tasks --project 1210197328049310

# Now operate on those tasks using the same --project flag
$ rustasana task 0 --project 1210197328049310
$ rustasana done 0 --project 1210197328049310
$ rustasana assign 1 me --project 1210197328049310
$ rustasana due 2 tomorrow --project 1210197328049310
$ rustasana comment 3 --project 1210197328049310
$ rustasana browse 4 --project 1210197328049310

# List tasks for a specific assignee
$ rustasana tasks --assignee 1234567890123456

# Operate on those tasks using --assignee flag
$ rustasana task 0 --assignee 1234567890123456
$ rustasana done 0 --assignee 1234567890123456
```

**Cache Contexts**:
- Default (no flags): Uses `~/.asana.cache` - your assigned tasks
- `--project <GID>`: Uses `~/.asana.cache.project.<GID>` - all tasks in that project
- `--assignee <GID>`: Uses `~/.asana.cache.<GID>` - tasks for that assignee

#### View Task Details

```bash
# View task at index 0
$ rustasana task 0

# View first task (default)
$ rustasana task

# View with comments and history
$ rustasana task 0 --verbose

# Output as JSON
$ rustasana task 0 --json

# View task from project context
$ rustasana task 0 --project 1210197328049310 --verbose
```

#### Complete a Task

```bash
$ rustasana done 2
Task marked as completed!

# Complete task from project context
$ rustasana done 42 --project 1210197328049310
```

#### Assign Task

```bash
# Assign task to yourself
$ rustasana assign 2 me
Task assigned to: John Doe

# Assign task to another user (by user GID)
$ rustasana assign 2 1234567890123456
Task assigned to: Jane Smith

# Unassign a task
$ rustasana assign 2 null
Task unassigned (set to no assignee)

# Alternative unassign syntax
$ rustasana assign 2 unassigned
Task unassigned (set to no assignee)

# Assign task from project context
$ rustasana assign 42 me --project 1210197328049310
```

#### Set Due Date

```bash
# Set specific date (YYYY-MM-DD format)
$ rustasana due 5 2024-08-21
Due date set to: 2024-08-21

# Use natural language
$ rustasana due 5 today
$ rustasana due 5 tomorrow

# Set due date for task in project context
$ rustasana due 10 tomorrow --project 1210197328049310
```

#### Add Comment

```bash
$ rustasana comment 2

# Add comment to task in project context
$ rustasana comment 42 --project 1210197328049310
```

This opens your default editor (set via `$EDITOR` environment variable). Write your comment, save, and close the editor.

#### Open Task in Browser

```bash
# Open task at index 1
$ rustasana browse 1

# Open first task (default)
$ rustasana browse
```

#### Download Attachment

```bash
# Download attachment by index
$ rustasana download 0 0
Downloading: document.pdf
To: document.pdf
Download complete!

# Download with custom output path
$ rustasana download 0 0 --output ~/Downloads/my-document.pdf

# Download by attachment GID
$ rustasana download 0 1234567890123456
```

First, view a task to see available attachments:
```bash
$ rustasana task 0

[ 2024-08-13 ] Write README
----------------------------------------
Attachments:
  [0] document.pdf (asana)
  [1] image.png (asana)
----------------------------------------
```

#### List Workspaces

```bash
$ rustasana workspaces
Workspaces:
* 4444444444444 My Project
  999999999999 Work
```

The `*` indicates your currently selected workspace.

### Command Aliases

All commands have short aliases for convenience:

| Command | Alias |
|---------|-------|
| `config` | `c` |
| `workspaces` | `w` |
| `projects` | `p` |
| `tasks` | `ts` |
| `task` | `t` |
| `comment` | `cm` |
| `browse` | `b` |
| `download` | `dl` |

### Help

```bash
$ rustasana --help
$ rustasana <command> --help
```

## Configuration

The configuration file is stored at `~/.asana.yml`:

```yaml
personal_access_token: 0/xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
workspace: "4444444444444"
```

### Cache

Task lists are cached at `~/.asana.cache` (or `~/.asana.cache.<filter>` for filtered views) for 5 minutes to reduce API calls and improve performance. Different cache files are used for different filters:

- `~/.asana.cache` - Tasks assigned to you (default)
- `~/.asana.cache.project.<project_gid>` - All tasks from a specific project
- `~/.asana.cache.<user_gid>` - Tasks for a specific user

## Environment Variables

- `EDITOR`: Your preferred text editor for comments (default: `vi` on Unix, `notepad` on Windows)
- `BROWSER`: Your preferred web browser (auto-detected if not set)

## Comparison with Go Version

This Rust implementation provides **100% feature parity** with the original Go version, with the following improvements:

- **Better error handling**: Uses Rust's `Result` type and the `anyhow` crate for detailed error messages
- **Type safety**: Strong typing prevents many runtime errors  
- **Memory safety**: Rust's ownership system ensures memory safety without garbage collection
- **Performance**: Compiled binary is fast and has low memory footprint
- **Enhanced UX**: Current workspace indicator in workspace listing
- **Attachment support**: View and download task attachments
- **Assignee visibility**: All task listings show assignee information
- **Project-based filtering**: View all tasks from any project (with automatic pagination)
- **Flexible filtering**: Filter by assignee or view entire project backlogs

All commands from the original Go implementation are supported, and config/cache files are compatible between both versions (note: filtered cache files and assignee display are new features in this Rust version).

## Project Structure

```
rustasana/
├── Cargo.toml          # Project dependencies and metadata
├── src/
│   ├── main.rs         # CLI entry point and command definitions
│   ├── api.rs          # Asana API client
│   ├── config.rs       # Configuration management
│   ├── models.rs       # Data structures for API responses
│   ├── utils.rs        # Utility functions
│   └── commands/       # Command implementations
│       ├── mod.rs      # Command module exports
│       ├── browse.rs   # Browse command
│       ├── comment.rs  # Comment command
│       ├── config.rs   # Config command
│       ├── done.rs     # Done command
│       ├── download.rs # Download command
│       ├── due.rs      # Due command
│       ├── task.rs     # Task command
│       ├── tasks.rs    # Tasks command
│       └── workspaces.rs # Workspaces command
└── README.md
```

## Dependencies

- **clap**: Command-line argument parsing with derive macros
- **reqwest**: HTTP client for making API requests
- **serde**: Serialization/deserialization framework
- **serde_json**: JSON support for API communication
- **serde_yaml**: YAML support for configuration files
- **anyhow**: Error handling and context
- **chrono**: Date and time utilities
- **dirs**: Cross-platform directory paths
- **tempfile**: Temporary file creation for editor

## Development

### Building

```bash
cargo build
```

### Running

```bash
cargo run -- <command>
```

### Testing

```bash
cargo test
```

### Linting

```bash
cargo clippy
```

## License

This project is inspired by [thash/asana](https://github.com/thash/asana). Please refer to the original project for licensing information.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Troubleshooting

### "Config file isn't set" error

Run `rustasana config` to set up your configuration.

### "Failed to open editor" error

Set your `EDITOR` environment variable:

```bash
export EDITOR=vim  # or nano, emacs, code, etc.
```

### "Task not found at index" error

Your cache might be stale, or you're using a different filter than when you listed tasks. Run the same command with `--refresh` to update the cache:

```bash
rustasana tasks --refresh  # For your tasks
rustasana tasks -p <project_gid> --refresh  # For project tasks
```

Note: Task indices are only valid within the same cache context. If you list tasks with `-p 123`, you must use that same cache to view tasks by index.

### "Project not found" error

Make sure to use a valid project GID. List all projects first:

```bash
rustasana projects
```

### "Assignee not found" error

When using `--assignee`, make sure to provide a valid user GID (not email or display name). You can find user GIDs by viewing a task in JSON format:

```bash
rustasana task 0 --json | grep -A 2 '"assignee"'
```

## Roadmap

Future enhancements:

- [ ] Create new tasks
- [ ] Assign tasks
- [ ] Add tags
- [ ] Project management
- [ ] Custom fields support
- [ ] Batch operations
- [ ] Interactive mode
- [ ] Shell completions

## Acknowledgments

- Original Go implementation by [thash](https://github.com/thash)
- [Asana API](https://developers.asana.com/docs)
