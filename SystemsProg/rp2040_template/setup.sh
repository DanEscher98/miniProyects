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