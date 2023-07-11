cd rust/acceptance-contracts
capsule build
echo "cp test data"
destination="build/debug"
build_dir="target/riscv64imac-unknown-none-elf/debug/*"
rm -rf $destination
mkdir -p "$destination"
for file in ${build_dir}; do
    echo ${file}
    if [[ -f "$file" && ! "$file" == *.* ]]; then
        cp "$file" "$destination"
    fi
done