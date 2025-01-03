# Event Enricher
Driver and service which help enrich event logs for detection purposes

# Build

## Prerequisites
All testing was done on Windows 11 with the following dependencies installed:
```
winget install --id Git.Git -e --source winget
winget install --id LLVM.LLVM -e --source winget
winget install --id Rustlang.Rustup -e --source winget
rustup install nightly
rustup default nightly
winget install --id Microsoft.VisualStudio.2022.Community --source winget --override "--add Microsoft.VisualStudio.Workload.NativeDesktop --add Microsoft.VisualStudio.Workload.Universal --includeRecommended --includeOptional"
winget install --source winget --exact --id Microsoft.WindowsWDK.10.0.22621
cargo install --locked cargo-make --no-default-features --features tls-native
```