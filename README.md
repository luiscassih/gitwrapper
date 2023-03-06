# GitWrapper
Personal repo of CLI apps written in Rust

`cargo run --bin gitwrapper set ../a/path/to/your/private/ssh-key`

`cargo run --bin gitwrapper-ssh git <your-git-command>`

## How to install
- Clone this repo
- `cargo build`
- `cargo install --path .`

if you want to replace your git for the entire session of your terminal, you can do 

`alias git=gitwrapper git`

## Example commands
- `gitwrapper set ../.ssh/my-personal-private-key`

- `gitwrapper git clone git@...my-personal-project.git`

