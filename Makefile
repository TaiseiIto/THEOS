BOOT_SECTOR=src/boot_sector/boot_sector.bin
BOOT_SOURCE=src/EFI/BOOT/BOOTX64.EFI
BOOT=$(THEOS_ROOT)/EFI/BOOT/BOOTX64.EFI
COPY=.bash/copy.sh
IMAGER=imager/target/release/imager
IMAGER_LOG=imager.log
THEOS=theos.img
THEOS_ROOT=root

# Build THEOS
all:
	make -C imager
	make -C src
	$(COPY) $(BOOT_SOURCE) $(BOOT)
	$(IMAGER) -b $(BOOT_SECTOR) -r $(THEOS_ROOT) > $(THEOS) 2> $(IMAGER_LOG)
	$(IMAGER) -i $(THEOS) >> $(IMAGER_LOG)

# Prepare a development environment on Docker and enter it.
# Usage: $ make docker
docker:
	make -C .docker

# Delete a development environment on Docker.
# Usage: $ make clean_docker
clean_docker:
	make clean -C .docker

# Delete a development environment on Docker and Prepare a new one and enter it.
# Usage: $ make rebuild_docker
rebuild_docker:
	make rebuild -C .docker

# Run THEOS on QEMU.
# Usage: $ make run
run:
	make -C .qemu

# Get permission to develop THEOS.
# Only developers can execute it and users don't have to do it.
# Usage: $ make permission GITHUB=<A path of ssh key to push to github.com> GITGPG=<A path of .gnupg directory to verify git commitment> CRATESIO=<A path of API key to log in crates.io>
permission:
	make permission -C .docker GITHUB=$(realpath $(GITHUB)) GITGPG=$(realpath $(GITGPG)) CRATESIO=$(realpath $(CRATESIO))

# Developers have to execute this before commitment to adjust source files in the repository.
# Usage: $ make adjust
adjust: delete_crlf tab_2_spaces

# This is called from target adjust.
# Convert CRLF to LF in sources in this repository.
delete_crlf:
	for i in $$(git grep -lr $$'\r'); do dos2unix $$i; done

# This is called from target adjust.
# Convert tabs to 2 spaces in all rust source files.
tab_2_spaces:
	for i in $$(find . -name "*.rs"); do expand -i -t 4 $$i | sponge $$i; done

