DISK = $(shell if [ ! -e "efs.img" ];then qemu-img create -f raw efs.img 100M;else echo "efs.img";fi|grep -o "efs.img")

test: $(DISK)
	cargo test -- --nocapture

clean:
	rm $(DISK)
