debug:
	cargo run --quiet -- --config example.toml into "$(cargo run --quiet -- --config example.toml list | fzf)"
