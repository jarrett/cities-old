# Used by Vagrant.

set -e
apt-get update
apt-get install -y curl git
curl -sSf https://static.rust-lang.org/rustup.sh | sh