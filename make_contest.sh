#!/bin/bash -l

if [ -z "$1" ]; then
    echo "Usage: ./make_contest.sh [CONTEST_NAME]"
    exit
fi

DIR="$1"

if ls "$DIR" >/dev/null 2>&1; then
    echo "$DIR already exists."
    exit
fi

cargo new "$DIR"
cd "$DIR" || exit

mkdir .vscode -p
cat <<EOF >.vscode/settings.json
{
    "rust-analyzer.check.command": "check"
}
EOF

mkdir .cargo -p
cat <<EOF >.cargo/config.toml
[build]
target-dir = "../target"
EOF

cargo add ac-library-rs@=0.1.1
cargo add num@=0.4.1
cargo add rand@=0.8.5
cargo add regex@=1.9.1
cargo add permutohedron@=0.2.4
cargo add superslice@=1.0.0
cargo add itertools@=0.11.0
cargo add proconio@=0.4.5 --features derive

mkdir -p src/bin
for prefix in {a..g}; do
    SRC=src/bin/$prefix.rs
    {
        printf "use proconio::*;\n\n"
        printf "fn main() {\n"
        printf "    \n"
        printf "}\n"
    } >>"$SRC"
    cargo equip --exclude-atcoder-crates --exclude-atcoder-202301-crates --minify libs --no-rustfmt --no-check --remove docs --remove comments --bin "$prefix" >/dev/null
done

cargo build
cargo build --release

code .