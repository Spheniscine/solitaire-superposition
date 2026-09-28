name="solitaire-superposition"

mv ./docs ./docs-old
dx bundle --release
compile_status=$?
if [[ $compile_status = 0 ]]; then
    mv "./target/dx/$name/release/web/public" ./docs
    sh ./compare_and_compress.sh ./docs-old/assets ./docs/assets
    rm -r ./docs-old
else
    mv ./docs-old ./docs
fi