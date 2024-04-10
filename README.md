# rustfmt-unstable

Rustfmt currently allows unstable options to be passed on the commandline only when used on stable. This behaviour is known to the rustfmt team and is not intended to change until version v2.0 in which a a new feature flag will be introduced that enables these features in the config. 
It is often desireable to use a rustfmt.toml config anyways, even with unstable features, to enable easier sharing of configuration across machines and developers. To aid in this purpose, this crate provides a way to parse the config and pass it as arguments instead.

## Usage
```bash
cargo install rustfmt-unstable
rustfmt-unstable
```

The rustfmt-unstable binary accepts a config file and then parses everything after the `--` as the actual command to be executed with the provided config. It will then pass `--config feature1=foo,feature2=bar,...` at the end of the provided command, if no `--config` is found in it already. If there is, rustfmt-unstable will instead ammend the feature list with the features in the config and replace it in-place. The command defaults to `cargo fmt --check --all --`.

The `--apply` flag is present if you simply wish to run `cargo fmt --all --`

Additional config options can be either provided through the --config option on rustfmt-unstable or through the --config option on the binary you wish to execute (after the first `--`).

The configuration file read can be specified by using the `--config-file` option. By default (with the `auto-resolve` feature enabled, which it is unless it is disabled) rustfmt-unstable tries to locate a `rustfmt.toml` file at the workspace root.

## Usage examples
### I wish to see a diff showing all the files not formatted correctly
```bash
rustfmt-unstable
```

### I wish to format all my unformatted files
```bash
rustfmt-unstable --apply
```

### I wish to provide another config file and format all my files accordingly
```bash
rustfmt-unstable --config-file my-config.toml --apply
```

### I just wish to wrap my existing `cargo fmt -- --config opt_a=foo,opt_b=bar` command to use the config file correctly
```bash
rustfmt-unstable -- cargo fmt -- --config opt_a=foo,opt_b=bar
```

### I wish to check that all my files were correctly formatted in ci
It's probably a good idea to check for overly long lines as well. I'd reccomend
```bash
rustfmt-unstable -- cargo fmt --check --all -- --config error_on_line_overflow=true
```
or if you don't wish to do that just
```bash
rustfmt-unstable
```
