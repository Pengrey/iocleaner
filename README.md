      
# iocleaner
IoC (Indicator of Compromise) remover framework.

`iocleaner` is a command-line tool that modifies source code projects to remove or alter IoCs based on a TOML configuration file. It uses regular expressions to find and replace patterns in specified files.

## Features

*   Regex-based IoC detection and replacement.
*   Configuration via TOML files.
*   Targeted file modification within a project.
*   Verbose and debug logging.
*   Option to ignore missing IoCs.

## Prerequisites

*   **Rust and Cargo:** Install from [rustup.rs](https://rustup.rs/).

## Installation
For the latest pre-compiled build, you can go to the [Releases](/releases) page on GitHub.

Alternatively, to build from source:

1.  Clone the repository:
    ```bash
    git clone https://github.com/Pengrey/iocleaner.git
    ```

2.  Navigate to the project directory:
    ```bash
    cd iocleaner
    ```

3.  Build the project:
    ```bash
    cargo build --release
    ```
    The executable will be at `target/release/iocleaner`.

## Configuration

`iocleaner` uses a TOML configuration file. Here's an example targeting [Evilginx v3.3.0](https://github.com/kgretzky/evilginx2/releases/tag/v3.3.0):

```toml
name = "Evilginx v3"                                    # Human-readable project name
description = "IoCs for Evilginx v3.3.0"                # Human-readable name for the IoC
version = "1.0.0"                                       # Version of the configuration file

[[ioc]]
path = "core/http_proxy.go"                             # Relative path to the file in the target project
name = "X-Evilginx Header Part 1"                       # Human-readable name for the IoC
description = "Removes part 1 of X-Evilginx header."    # Human-readable description for the IoC
regex = '^\s*o_host := req\.Host$'                      # Regex to find the IoC
replacement = ""                                        # Replacement string (empty to remove)

[[ioc]]
path = "core/http_proxy.go"
name = "X-Evilginx Header Part 2"
description = "Removes part 2 of X-Evilginx header."
regex = '^\s*req\.Header\.Set\(p\.getHomeDir\(\), o_host\)$'
replacement = ""

[[ioc]]
path = "core/http_proxy.go"
name = "X-Evilginx Header Part 3"
description = "Removes part 3 of X-Evilginx header (function definition)."
regex = '^func \(p \*HttpProxy\) getHomeDir\(\) string \{\n\s*return strings\.Replace\(HOME_DIR, "\.e", "X-E", 1\)\n\}'
replacement = ""
```

**Key Configuration Fields:**
*   **Global:** `name`, `description`, `version` (for the config file itself).
*   **`[[ioc]]` block:**
    *   `path`: File to modify (relative to project root).
    *   `name`: IoC identifier.
    *   `description`: Details about the IoC.
    *   `regex`: Pattern to find.
    *   `replacement`: String to replace with (empty for deletion).

## Usage

**⚠️ WARNING: `iocleaner` modifies files in place. Always back up your project or work on a copy!**

### Command-Line Options

```bash
$ ./target/release/iocleaner -h
Usage: iocleaner [OPTIONS] --project <PROJECT_PATH> --config <CONFIG_PATH>

Options:
  -p, --project <PROJECT_PATH>  Path to the root directory of the project to clean
  -c, --config <CONFIG_PATH>    Path to the TOML IoC configuration file
  -d, --debug                   Enable debug logging
  -v, --verbose                 Enable verbose logging
  -i, --ignore                  Ignore IoCs presence check (continue if IoC not found)
  -h, --help                    Print help
  -V, --version                 Print version
$
```

### Basic Example

1.  Save your IoC definitions to a file (e.g., `my_cleaner_config.toml`).
2.  Run `iocleaner`:
    ```bash
    ./target/release/iocleaner --project /path/to/your/project \
                               --config /path/to/my_cleaner_config.toml
    ```
    Use `-v` for verbose output or `-d` for debug output.

## How it Works

`iocleaner` reads the project directory and the IoC configuration file. For each IoC defined:
1.  It locates the target file specified by `path`.
2.  Reads the file content.
3.  Uses the `regex` to find occurrences of the IoC.
4.  Replaces them with the `replacement` string.
5.  Overwrites the original file with the modified content.

## Use Cases

*   **Red Teaming:** Modify tools to evade signature-based detection.
*   **Malware Analysis:** Neutralize specific IoCs in samples for safer analysis.
*   **Code Sanitization:** Remove unwanted patterns or metadata.

## Contributing

Contributions are welcome! Please fork the repository, create a feature branch, make your changes, and submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.