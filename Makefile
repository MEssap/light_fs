DISK = $(shell if [ ! -e "disk.img" ];then qemu-img create -f raw disk.img 100M;else echo "disk.img";fi|grep -o "disk.img")

test: $(DISK)
	cargo test -- --nocapture

clean:
	rm $(DISK)
