EFS = $(shell if [ ! -e "efs.img" ];then qemu-img create -f raw efs.img 100M;else echo "efs.img";fi|grep -o "efs.img")
FAT32 = $(shell if [ ! -e "fat32.img" ];then qemu-img create -f raw fat32.img 100M;mkfs.fat -F 32 fat32.img;else echo "fat32.img";fi|grep -o "fat32.img")

disk: $(EFS) $(FAT32)

test:
	make disk
	cargo test -- --nocapture

clean:
	cargo clean
	rm $(EFS) $(FAT32)
