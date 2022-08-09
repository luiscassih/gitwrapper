# lainapps
Personal repo of CLI apps written in Rust

`cargo run --bin laingit set ../a/path/to/your/private/ssh-key`

`cargo run --bin lainssh git <your-git-command>`

## How to install
- Clone this repo
- `cargo build`
- `cargo install --path .`

if you want to replace your git for the entire session of your terminal, you can do 

`alias git=laingit`

## Example commands
- `laingit set ../.ssh/my-personal-private-key`

- `laingit git clone git@...my-personal-project.git`

