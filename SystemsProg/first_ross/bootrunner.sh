cd /home/dany98/.cargo/registry/src/github.com-1ecc6299db9ec823/bootloader-0.10.9/ || return

cargo builder --kernel-manifest ~/Working/Proyects/SystemsProg/first_ross/Cargo.toml --kernel-binary ~/Working/Proyects/SystemsProg/first_ross/target/debug/first_ross --target-dir ~/Working/Proyects/SystemsProg/first_ross/target/ --out-dir ~/Working/Proyects/SystemsProg/first_ross/target/debug/

cd ~/Working/Proyects/SystemsProg/first_ross/ || return
qemu-system-x86_64 -drive format=raw,file=target/x86_64-ross/debug/bootimage-first_ross.bin
