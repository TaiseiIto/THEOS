# Build THEOS
all:
	make -C src
	make -C imager
	imager/target/release/imager src/boot_sector/boot_sector.bin disk > theos.img 2> imager_output.txt

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

# Get permission to develop THEOS.
# Only developers can execute it and users don't have to do it.
# Usage: $ make permission GITHUB=<A path of ssh key to push to github.com> GITGPG=<A path of .gnupg directory to verify git commitment> CRATESIO=<A path of API key to log in crates.io>
permission:
	make permission -C .docker GITHUB=$(realpath $(GITHUB)) GITGPG=$(realpath $(GITGPG)) CRATESIO=$(realpath $(CRATESIO))

# Convert CRLF to LF in sources in this repository.
# Developers have to execute it before commitment to prevent that CRLFs mix into source files in this repository.
# Usage: $ make delete_crlf
delete_crlf:
	for i in $$(git grep -lr $$'\r'); do dos2unix $$i; done

