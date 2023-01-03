run_docker:
	make -C .docker

clean_docker:
	make clean -C .docker

rebuild_docker:
	make rebuild -C .docker

empowerment:
	make empowerment -C .docker GITHUB=$(realpath $(GITHUB)) GITGPG=$(realpath $(GITGPG)) CRATESIO=$(realpath $(CRATESIO))

delete_crlf:
	for i in $$(git grep -lr $$'\r'); do dos2unix $$i; done

