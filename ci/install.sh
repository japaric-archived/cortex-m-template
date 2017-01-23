set -ex

main() {
    curl https://sh.rustup.rs -sSf | \
        sh -s -- -y --default-toolchain $TRAVIS_RUST_VERSION

    local tag=$(git ls-remote --tags --refs --exit-code https://github.com/japaric/cross \
                    | cut -d/ -f3 \
                    | grep -E '^v0\.1\.[0-9]+$' \
                    | sort --version-sort \
                    | tail -n1)

    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --force \
           --git japaric/cross \
           --tag $tag \
           --target x86_64-unknown-linux-gnu
}

main
