DOCKER = docker
DOCKER_IMAGE = theos-devenv
DOCKER_IMAGE_TAG = latest
DOCKER_CONTAINER = theos-devenv
DOCKER_SHELL = /bin/sh
GENIMG = genimg/genimg

$(GENIMG):
	make -C $(dir $@)

clean:
	make clean -C $(dir $(GENIMG))

clean-devenv:
	./script/clean-devenv.sh $(DOCKER) $(DOCKER_IMAGE) $(DOCKER_CONTAINER)

devenv:
	./script/devenv.sh $(DOCKER) $(DOCKER_IMAGE) $(DOCKER_IMAGE_TAG) $(DOCKER_CONTAINER)

rebuild: clean
	make

rebuild-devenv: clean-devenv
	make devenv

.PHONY: build-devenv clean rebuild-devenv remove-devenv run-devenv start-devenv stop-devenv
