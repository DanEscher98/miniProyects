project="$HOME/Working/Proyects/SystemsProg/first_ross"

cd /home/dany98/.cargo/registry/src/github.com-1ecc6299db9ec823/bootloader-0.10.9/ || return

cargo builder \
	--kernel-manifest "$project"/Cargo.toml \
	--kernel-binary "$project"/target/debug/first_ross \
	--target-dir "$project"/target/ \
	--out-dir "$project"/target/debug/

cd "$project" || return
qemu-system-x86_64 \
	-drive format=raw,file=target/x86_64-ross/debug/bootimage-first_ross.bin
