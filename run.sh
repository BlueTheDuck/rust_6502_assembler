rm out.json
rustfmt ./**/*.rs;RUSTFLAGS="$RUSTFLAGS -A unused_imports -A unused_variables -A unused_assignments -A dead_code" cargo run >> out.json