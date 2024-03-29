DISK = $(shell if [ ! -e "disk.img" ];then qemu-img create -f raw disk.img 100M;mkfs.ext4 disk.img;else echo "disk.img";fi|grep -o "disk.img")

test: $(DISK)
	cargo test -- --nocapture

clean:
	rm $(DISK)
