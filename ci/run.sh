set -ex

run() {
    case $TARGET in
        thumbv*-none-eabi*)
            xargo build --target $TARGET --example app
            xargo build --target $TARGET --release --example app
            ;;
        *)
            cargo build --target $TARGET
            cargo build --target $TARGET --release
            ;;
    esac
}

run
