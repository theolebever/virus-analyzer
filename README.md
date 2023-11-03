# Virus Analyzer

Virus Analyzer is a Rust-based project that allows for the analysis of files for potential malware by utilizing various Docker images from different antivirus vendors. It orchestrates the execution of Docker containers, each running a different antivirus software, to scan files for malware and report the findings.

## Prerequisites

- Rust: You can install Rust from the [official website](https://rust-lang.org).
- Docker: Install Docker from the [official website](https://www.docker.com).

## Usage

1. Clone the repository:

```bash
git clone https://github.com/your-username/virus-analyzer.git
cd virus-analyzer
```
2. Build the project:

```
cargo build --release
```

3. Run the project:

```
cargo run --release --path /path/to/malware/folder
```

Make sure to replace /path/to/malware/folder with the actual path to the folder containing the files you want to analyze.