# rustfmt-unstable

Rustfmt currently allows unstable options to be passed on the commandline only when used on stable. This behaviour is known to the rustfmt team and is not intended to change until version v2.0 in which a a new feature flag will be introduced that enables these features in the config. 
It is often desireable to use a rustfmt.toml config anyways, even with unstable features, to enable easier sharing of configuration across machines and developers. To aid in this purpose, this crate provides a way to parse the config and pass it as arguments instead.

## Usage
```bash
cargo install rustfmt-unstable
rustfmt-unstable
```

The rustfmt-unstable binary accepts a config file and then parses everything after the `--` as the actual command to be executed with the provided config. It will then pass `--config feature1=foo,feature2=bar,...` at the end of the provided command, if no `--config` is found in it already. If there is, rustfmt-unstable will instead ammend the feature list with the features in the config and replace it in-place. The command defaults to `cargo fmt --check --all --`.

Additional config options can be either provided through the --config option on rustfmt-unstable or through the --config option on the binary you wish to execute (after the first `--`).

The configuration file read can be specified by using the `--config-file` option. By default (with the `auto-resolve` feature enabled, which it is unless it is disabled) rustfmt-unstable tries to locate a `rustfmt.toml` file at the workspace root.

## Troubleshooting
### `Error: unexpected --config found` when using cargo fmt
This happens when you use `rustfmt-unstable --config-file rustfmt.toml -- cargo fmt` instead of `rustfmt-unstable --config-file rustfmt.toml -- cargo fmt --` - the last `--` is relevant, as rustfmt-unstable just appends a --config to the provided command, and as such that needs to be a valid argument. cargo-fmt doesn't accept this argument, but will pass everything after the `--` to the rustfmt binary directly instead. As such we need a `--` at the end, to ensure rustfmt-unstable passes it in correctly.
