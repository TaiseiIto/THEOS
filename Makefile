DOCKER = docker
DOCKER_IMAGE = theos-devenv
DOCKER_IMAGE_TAG = latest
DOCKER_CONTAINER = theos-devenv
DOCKER_SHELL = /bin/sh
GENIMG = genimg/genimg

ifeq ($(OS), Windows_NT)
BLANK =
DELIMITER = \$(BLANK)
SCRIPT_PREFIX = 
SCRIPT_SUFFIX = .bat
else
DELIMITER = /
SCRIPT_PREFIX = ./
SCRIPT_SUFFIX = .sh
endif

$(GENIMG):
	make -C $(dir $@)

clean:
	make clean -C $(dir $(GENIMG))

clean-devenv:
	$(SCRIPT_PREFIX)script$(DELIMITER)clean-devenv$(SCRIPT_SUFFIX) $(DOCKER) $(DOCKER_IMAGE) $(DOCKER_CONTAINER)

devenv:
	$(SCRIPT_PREFIX)script$(DELIMITER)devenv$(SCRIPT_SUFFIX) $(DOCKER) $(DOCKER_IMAGE) $(DOCKER_IMAGE_TAG) $(DOCKER_CONTAINER)

rebuild: clean
	make

rebuild-devenv: clean-devenv
	make devenv

.PHONY: build-devenv clean rebuild-devenv remove-devenv run-devenv start-devenv stop-devenv

