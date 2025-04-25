// src/main.rs
use clap::{CommandFactory, Parser, Subcommand};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error as IoError, ErrorKind, Write};
use std::path::Path;
use std::process;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Parser)]
#[clap(name = "forc-install", about = "CLI tool to manage GitHub dependencies in Forc.toml")]
#[clap(arg_required_else_help = true)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,

    /// GitHub repository link in owner/repo format
    #[clap(value_name = "GITHUB_OWNE/REPO")]
    github_repo: Option<String>,

    /// Package name (optional)
    #[clap(short = 'p', long = "package")]
    package: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Remove a dependency from Forc.toml
    #[clap(name = "rm")]
    Remove {
        /// GitHub repository link in owner/repo format
        github_repo: String,

        /// Package name (optional)
        #[clap(short = 'p', long = "package")]
        package: Option<String>,
    },
}

fn main() {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Process the command
    let result = match &cli.command {
        Some(Commands::Remove { github_repo, package }) => {
            // Remove dependency
            remove_dependency(github_repo, package)
        }
        None => {
            // If no subcommand but a GitHub repo is provided, add dependency
            if let Some(github_repo) = &cli.github_repo {
                add_dependency(github_repo, &cli.package)
            } else {
                // Display help if no arguments provided
                Cli::command().print_help().map_err(|e| e.into())
            }
        }
    };

    // Handle errors
    if let Err(err) = result {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

fn validate_github_repo(repo: &str) -> Result<()> {
    let parts: Vec<&str> = repo.split('/').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return Err(format!("Invalid GitHub repository format: '{}'. Expected format: 'owner/repo'", repo).into());
    }
    Ok(())
}

fn add_dependency(github_repo: &str, package: &Option<String>) -> Result<()> {
    // Validate GitHub repository format
    validate_github_repo(github_repo)?;
    
    // Find Forc.toml file
    let toml_path = find_forc_toml()?;
    
    // Determine the package name (either from argument or from repo name)
    let package_name = match package {
        Some(pkg) => pkg.clone().replace('-', "_"),
        None => {
            let parts: Vec<&str> = github_repo.split('/').collect();
            parts[1].replace('-', "_")
        }
    };
    
    // GitHub URL format
    // Keep the original repo format for the URL, but sanitize for TOML key
    let github_url = format!("https://github.com/{}", github_repo);
    
    // Check if dependency already exists
    let file = File::open(&toml_path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.contains(&github_url) {
            if package.is_none() {
                // For auto-named packages, just check URL
                return Err(format!("Dependency for {} already exists in Forc.toml", github_url).into());
            } else if line.trim().starts_with(&format!("{} = ", package_name)) {
                // For explicitly named packages, check both the name and URL
                return Err(format!("Dependency for {} with package name '{}' already exists in Forc.toml", 
                                  github_url, package_name).into());
            }
        }
    }
    
    // Format the dependency line to add
    let dependency_line = format!("{} = {{ git = \"{}\" }}\n", package_name, github_url);
    
    // Open the file in append mode
    let mut file = OpenOptions::new()
        .append(true)
        .open(toml_path)?;
    
    // Write the dependency
    file.write_all(dependency_line.as_bytes())?;
    
    println!("Added dependency: {} from {}", package_name, github_url);
    Ok(())
}

fn remove_dependency(github_repo: &str, package: &Option<String>) -> Result<()> {
    // Validate GitHub repository format
    validate_github_repo(github_repo)?;
    
    // Find Forc.toml file
    let toml_path = find_forc_toml()?;

    let package_name = match package {
        Some(pkg) => pkg.clone().replace('-', "_"),
        None => {
            let parts: Vec<&str> = github_repo.split('/').collect();
            parts[1].replace('-', "_")
        }
    }; 
    
    // Read the file content
    let file = File::open(&toml_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<std::result::Result<_, _>>()?;
    
    // Find and filter out the dependency line
    let mut found = false;
    let new_content: Vec<String> = lines
        .into_iter()
        .filter(|line| {
            // Check if line contains the github repo
            if line.contains(&package_name) {
                // If package is specified, check if it's at the beginning of the line
                if let Some(pkg) = package {
                    // Convert hyphens to underscores for comparison
                    let pkg_normalized = pkg.replace('-', "_");
                    if line.trim().starts_with(&format!("{} = ", pkg_normalized)) {
                        found = true;
                        return false;
                    }
                } else {
                    found = true;
                    return false;
                }
            }
            true
        })
        .collect();
    
    if found {
        // Write the new content back to the file
        let mut file = File::create(&toml_path)?;
        for line in new_content {
            writeln!(file, "{}", line)?;
        }
        println!("Removed dependency from {}", package_name);
        Ok(())
    } else {
        Err(format!("Dependency not found in Forc.toml: {}", package_name).into())
    }
}

fn find_forc_toml() -> Result<String> {
    let current_dir = std::env::current_dir()?;
    let forc_toml_path = current_dir.join("Forc.toml");
    
    if Path::new(&forc_toml_path).exists() {
        Ok(forc_toml_path.to_string_lossy().to_string())
    } else {
        Err(IoError::new(ErrorKind::NotFound, "Forc.toml file not found in the current directory").into())
    }
}