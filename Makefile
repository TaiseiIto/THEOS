DOCKER = docker
DOCKER_FILE = docker/Dockerfile
DOCKER_IMAGE = theos-devenv
DOCKER_IMAGE_TAG = latest
DOCKER_CONTAINER = theos-devenv
DOCKER_SHELL = /bin/sh

build-devenv:
	$(DOCKER) build --no-cache -t $(DOCKER_IMAGE):$(DOCKER_IMAGE_TAG) $(dir $(DOCKER_FILE))

rebuild-devenv: remove-devenv
	make build-devenv

remove-devenv:
	$(DOCKER) rm $(DOCKER_CONTAINER)
	$(DOCKER) rmi $(DOCKER_IMAGE)

run-devenv:
	$(DOCKER) run --name $(DOCKER_CONTAINER) -i -t $(DOCKER_IMAGE)

start-devenv:
	$(DOCKER) start $(DOCKER_CONTAINER)

stop-devenv:
	$(DOCKER) stop $(DOCKER_CONTAINER)

.PHONY: build-devenv rebuild-devenv remove-devenv run-devenv start-devenv stop-devenv
