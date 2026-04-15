# Rustasana

A blazingly fast command-line interface for [Asana](https://asana.com/) written in Rust. This is a Rust implementation inspired by [thash/asana](https://github.com/thash/asana).

## Features

- 🚀 Fast and efficient CLI for Asana
- 💾 Local caching for quick task listing
- 📝 Add comments via your favorite editor
- 🌐 Open tasks directly in your browser
- ✅ Mark tasks as complete
- 📅 Set due dates with natural language (today, tomorrow)
- 🔍 View task details with verbose mode

## Installation

### Prerequisites

- Rust 1.70 or higher ([Install Rust](https://www.rust-lang.org/tools/install))

### Build from source

```bash
cd rustasana
cargo build --release
```

The binary will be available at `target/release/rustasana`.

### Install globally

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

#### List Tasks

```bash
# List all your tasks
$ rustasana tasks

# Or use the short alias
$ rustasana ts

# Bypass cache
$ rustasana ts --no-cache

# Refresh cache
$ rustasana ts --refresh
```

Output:
```
 0 [ 2024-08-13 ] Write README
 1 [ 2024-08-18 ] Buy gift for coworkers
 2 [ 2024-08-29 ] Read "Unweaving the Rainbow"
 3 [            ] haircut
```

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
```

#### Complete a Task

```bash
$ rustasana done 2
Task marked as completed!
```

#### Set Due Date

```bash
# Set specific date (YYYY-MM-DD format)
$ rustasana due 5 2024-08-21
Due date set to: 2024-08-21

# Use natural language
$ rustasana due 5 today
$ rustasana due 5 tomorrow
```

#### Add Comment

```bash
$ rustasana comment 2
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

Task lists are cached at `~/.asana.cache` for 5 minutes to reduce API calls and improve performance.

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

All commands from the original Go implementation are supported, and config/cache files are fully compatible between both versions.

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

Your cache might be stale. Run `rustasana tasks --refresh` to update the cache.

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
