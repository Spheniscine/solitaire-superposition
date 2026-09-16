cp -r ./docs ./docs-old
rm -r ./docs
dx bundle --release
mv ./target/dx/solitaire-superposition/release/web/public ./docs
sh ./compare_and_compress.sh ./docs-old/assets ./docs/assets
rm -r ./docs-old