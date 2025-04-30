# forc-install

A command-line interface (CLI) tool written in Rust to manage GitHub dependencies in your `Forc.toml` file.

## Features

- Add GitHub dependencies to your `Forc.toml` file
- Remove existing dependencies
- Automatic package name generation from repository name
- Support for custom package names

## Installation


```bash
# Clone the repository
git clone https://github.com/darthbenro008/forc-install

# go into directory
cd forc-install

# install
cargo install --path .

# check if plugin is detected
forc plugins

# usage
forc install owner/repo
```

## Usage

### Adding Dependencies

Add a dependency using the repository name as the package name:
```bash
forc-install owner/repo
```

Add a dependency with a custom package name:
```bash
forc-install owner/repo -p custom_name
```

### Removing Dependencies

Remove a dependency:
```bash
forc-install rm owner/repo
```

Remove a dependency with a specific package name:
```bash
forc-install rm owner/repo -p custom_name
```

### Help

Display help information:
```bash
forc-install --help
```

## Examples

1. Add a dependency using the repository name:
```bash
forc-install fuel-labs/sway
# Adds: sway = { git = "https://github.com/fuel-labs/sway" }
```

2. Add a dependency with a custom package name:
```bash
forc-install fuel-labs/sway -p my_sway
# Adds: my_sway = { git = "https://github.com/fuel-labs/sway" }
```

3. Remove a dependency:
```bash
forc-install rm fuel-labs/sway
```

## Notes

- The tool searches for `Forc.toml` in the current directory
- Package names are automatically converted from kebab-case to snake_case
- If a dependency already exists, the tool will prevent duplicate entries
- When removing dependencies, the tool matches both the package name and GitHub URL

## Error Handling

The tool will display appropriate error messages for common issues:
- Invalid GitHub repository format
- Missing `Forc.toml` file
- Duplicate dependencies
- Non-existent dependencies when trying to remove them

## License

Copyright 2025 Hemanth Krishna

Licensed under MIT License : <https://opensource.org/licenses/MIT>

<p align="center">Made with ❤ , single can of redbull</p>
