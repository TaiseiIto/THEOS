run_docker:
	make -C .docker

clean_docker:
	make clean -C .docker

rebuild_docker:
	make rebuild -C .docker

empowerment:
	make empowerment -C .docker GITHUB=$(realpath $(GITHUB)) GITGPG=$(realpath $(GITGPG)) CRATESIO=$(realpath $(CRATESIO))

