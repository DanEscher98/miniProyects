# Install the toolchain
sudo apt install git gdb-multiarch
sudo apt install automake autoconf build-essential \
	texinfo libtool libftdi-dev libusb-1.0-0-dev

# Setup Open-OnChipDebugger
git clone https://github.com/raspberrypi/openocd.git --branch picoprobe --depth=1
cd openocd || return
./configure --disable-werror --enable-ftdi --enable-sysfsgpio --enable-bcm2835gpio --enable-picoprobe
make -j4
sudo make install

# https://datasheets.raspberrypi.org/soft/picoprobe.uf2
# Copy the picoprobe.uf2 into one of the RaspberryPico

# Setting Picoprobe user Device Access
picoid=$(lsusb | grep -i pico | awk '{ print $6 }')		# 2e8a:0004
idVendor=$(echo "$picoid" | awk -F: '{ print $1 }')
idProduct=$(echo "$picoid" | awk -F: '{ print $2 }')
echo "# RaspberryPi Picoprobe" | sudo tee -a /etc/udev/rules.d/99-openocd.rules
echo "ATTRS{idVendor}==\"$idVendor\"," \
	"ATTRS{idProduct}==\"$idProduct\", MODE:=\"0666\"" \
	| sudo tee -a /etc/udev/rules.d/99-openocd.rules
sudo udevadm control --reload-rules

# Change cargo to nightly
rustup override set nightly
rustup target install thumbv6m-none-eabi
cargo install --git https://github.com/rp-rs/probe-run --branch main
cargo install flip-link

# Load the app onto the target with gdb
# Connect the raspberry pico
cd "$HOME"/openocd || return
openocd -f tlc/interface/picoprobe.cfg -f tlc/target/rp2040.cfg -s tcl
cd - || return
gdb-multiarch -q -ex "target extended-remote :3333" \
	target/thumbv6m-none-eabi/debug/rp2040_template
# In the GDB shell
# (gdb) load
# (gdb continue)
