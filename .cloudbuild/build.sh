# Update rust toolchain
rustup toolchain uninstall stable
rustup toolchain install stable --no-self-update
rustup default stable

# Run `cargo build --release`
cargo build --release --locked

# Package 
if [-z "${version}"]; then
  echo "The version environment variable is not set."
  exit 1
fi

ARCH=$(uname -m)
FILENAME="build-inspector-${version}-${ARCH}.tar.gz"

cd target/release
tar -czvf "${FILENAME}" build-inspector