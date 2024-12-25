# Justfile (Convenience Command Runner)

# rust vars
# RUST_LOG:= 'debug'
RUST_BACKTRACE:= '1'
RUSTFLAGS:='--cfg tokio_unstable'
TOML_VERSION:=`rg '^version = ".*"' Cargo.toml | sd '.*"(.*)".*' '$1'`
# just path vars
HOME_DIR := env_var('HOME')
LOCAL_ROOT := justfile_directory()
INVOCD_FROM := invocation_directory()
INVOC_IS_ROOT := if INVOCD_FROM == LOCAL_ROOT { "true" } else { "false" }
# custom vars
FROZE_SHA_REGEX := 'FROZE_[a-fA-F0-9]{64}_FROZE-'
# ANSI Color Codes for use with echo command
NC := '\033[0m'     # No Color
CYN := '\033[0;36m' # Cyan
BLU := '\033[0;34m' # Blue
GRN := '\033[0;32m' # Green
PRP := '\033[0;35m' # Purple
BRN := '\033[0;33m' # Brown

# Default, lists commands.
_default:
        @just --list --unsorted

# Initialize repository.
[confirm]
init: && list-external-deps _gen-env _gen-git-hooks
    cargo clean
    cargo build
    cargo doc --all-features --document-private-items

# Add a package to workspace // update-comment: the heck am I doing adding, removing, then using cargo-generate?
newday day_digits:
    cargo generate --path ./.support_data/cargo_generate_templates/cg_template__new_day --name day{{day_digits}}


# Linting, formatting, typo checking, etc.
check:
    cargo clippy
    cargo fmt
    typos
    committed

# Show docs.
docs:
    rustup doc
    rustup doc --std
    cargo doc --all-features --document-private-items --open

# Update Rust-crates, non-breaking updates only.
update-soft:
    cargo update --verbose

# Update Rust-crates, first minor, then breaking changes.
[confirm]
update-hard: update-soft
    cargo update --verbose --breaking -Z unstable-options

# All tests, little feedback unless issues are detected.
test:
    cargo test --doc
    cargo nextest run --cargo-quiet --cargo-quiet

# Runtests for a specific package.
testp package="":
    cargo test --doc --quiet --package {{package}}
    cargo nextest run --cargo-quiet --cargo-quiet --package {{package}}

# Run a specific test with output visible. (Use '' for test_name to see all tests and set log_level)
test-view test_name="" log_level="error":
    @echo "'Fun' Fact; the '--test' flag only allows integration test selection and will just fail on unit tests."
    RUST_LOG={{log_level}} cargo test {{test_name}} -- --nocapture

# Run a specific test with NEXTEST with output visible. (Use '' for test_name to see all tests and set log_level)
testnx-view test_name="" log_level="error":
    @echo "'Fun' Fact; the '--test' flag only allows integration test selection and will just fail on unit tests."
    RUST_LOG={{log_level}} cargo nextest run {{test_name}} --no-capture

# All tests, little feedback unless issues are detected.
test-whisper:
    cargo test --doc --quiet
    cargo nextest run --cargo-quiet --cargo-quiet --status-level=leak

# Run performance analysis on a package.
perf package *args:
    cargo build --profile profiling --bin {{package}};
    hyperfine --export-markdown=.output/profiling/{{package}}_hyperfine_profile.md './target/profiling/{{package}} {{args}}' --warmup=3 --shell=none;
    samply record --output=.output/profiling/{{package}}_samply_profile.json --iteration-count=3 ./target/profiling/{{package}} {{args}};

# Run hyperfine performance analysis on a package.
perf-hyper package *args:
    cargo build --profile profiling --bin {{package}};
    hyperfine --export-markdown=.output/profiling/{{package}}_hyperfine_profile.md './target/profiling/{{package}} {{args}}' --warmup=1 --shell=none;

# Possible future perf compare command.
#
perf-compare-info:
    @echo "Use hyperfine directly:\n{{GRN}}hyperfine{{NC}} {{BRN}}'cmd args'{{NC}} {{BRN}}'cmd2 args'{{NC}} {{PRP}}...{{NC}} --warmup=3 --shell=none --export-markdown=.output/profiling/..."
    @echo "Preceded by {{GRN}}cargo build --profile profiling --bin <package>;{{NC}}"

# List dependencies. (This command has dependencies.)
list-external-deps:
    @echo "{{CYN}}List of external dependencies for this command runner and repo:"
    xsv table ad_deps.csv

# Info about Rust-Compiler, Rust-Analyzer, Cargo-Clippy, and Rust-Updater.
rust-meta-info:
    rustc --version
    rust-analyzer --version
    cargo-clippy --version
    rustup --version
# ######################################################################## #

# Print reminder: how to set env vars that propagate to child shells.
_remind-setenv:
    @ echo '{{GRN}}set -a{{NC}}; {{GRN}}source {{BLU}}.env{{NC}}; {{GRN}}set +a{{NC}}'

# ######################################################################## #

# Generate .env file from template, if .env file not present.
_gen-env:
    @ if [ -f '.env' ]; then echo '`{{BRN}}.env{{NC}}` exists, {{PRP}}skipping creation{{NC}}...' && exit 0; else cp -n .support_data/template.env .env; echo "{{BLU}}.env{{NC}} created from template. {{GRN}}Please fill in the necessary values.{{NC}}"; echo "e.g. via 'nvim .env'"; fi

# Attempt to add all git-hooks. (no overwrite)
_gen-git-hooks: _gen-precommit-hook _gen-commitmsg-hook

# Attempt to add `pre-commit` git-hook. (no overwrite)
_gen-precommit-hook:
    @ if [ -f '.git/hooks/pre-commit' ]; then echo '`.git/hooks/{{BRN}}pre-commit{{NC}}` exists, {{PRP}}skipping creation{{NC}}...' && exit 0; else cp -n .support_data/git_hooks/pre-commit .git/hooks/pre-commit; chmod u+x .git/hooks/pre-commit; echo live "{{BLU}}pre-commit{{NC}} hook added to {{GRN}}.git/hooks{{NC}} and set as executable"; fi

# Attempt to add `commit-msg` git-hook. (no overwrite)
_gen-commitmsg-hook:
    @ if [ -f '.git/hooks/commit-msg' ]; then echo '`.git/hooks/{{BRN}}commit-msg{{NC}}` exists, {{PRP}}skipping creation{{NC}}...' && exit 0; else cp -n .support_data/git_hooks/commit-msg .git/hooks/commit-msg; chmod u+x .git/hooks/commit-msg; echo live "{{BLU}}commit-msg{{NC}} hook added to {{GRN}}.git/hooks{{NC}} and set as executable"; fi

# ######################################################################## #

# Freeze! For your safety.
_freeze file:
	mv -iv {{file}} FROZE_{{sha256(file)}}_FROZE-{{file}} | rg {{file}}

# Unfreeze a file. (removes 'FROZE...FROZE-' tag from filename)
_thaw file:
	echo {{file}} | sd '{{FROZE_SHA_REGEX}}' '' | xargs mv -iv {{file}}

# Search local files through ice.
_arctic-recon iceless_name:
	fd --max-depth 1 '{{FROZE_SHA_REGEX}}{{iceless_name}}' | rg {{iceless_name}}


# ######################################################################## #

# Speak Funny to Me!
_uu:
	echo {{uuid()}}

# Say my name.
_sha file:
	echo {{sha256_file(file)}}

# Example function for syntax reference
_example-file-exists-test file:
    echo {{ if path_exists(file) == "true" { "hello" } else { "goodbye" } }}

# ######################################################################## #


# # Clean up cargo build artifacts.
# [confirm]
# teardown:
#     cargo clean

# # Auto-fix some errors picked up by check. (Manual exclusion of data folder as additional safeguard.)
# [confirm]
# fix:
#      typos --exclude '*/data/*' --write-changes

# # Run git hook.
# git-hook hook='pre-commit':
#     git hook run {{hook}}

# # Watch a file: compile & run on changes.
# watch file_to_run:
#     cargo watch --quiet --clear --exec 'run --quiet --example {{file_to_run}}'
