set -ex

main() {
    cross build --target $TARGET --example app
    cross build --target $TARGET --release --example app

    cross build --features semihosting --target $TARGET --example semihosting
    cross build --features semihosting --target $TARGET --release --example semihosting
}

if [ $TRAVIS_BRANCH != master ] || [ $TRAVIS_EVENT_TYPE = cron ]; then
    main
fi
