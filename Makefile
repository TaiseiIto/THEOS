DOCKER = docker
DOCKER_IMAGE = theos-devenv
DOCKER_IMAGE_TAG = latest
DOCKER_CONTAINER = theos-devenv
DOCKER_SHELL = /bin/sh

build-devenv:
	$(DOCKER) build --no-cache -t $(DOCKER_IMAGE):$(DOCKER_IMAGE_TAG) .

login-devenv:
	$(DOCKER) attach $(DOCKER_CONTAINER)

rebuild-devenv: remove-devenv
	make build-devenv

remove-devenv:
	if [ -n "$$($(DOCKER) ps -a | grep $(DOCKER_CONTAINER))" ]; then	\
		$(DOCKER) rm $(DOCKER_CONTAINER);				\
	fi;									\
	if [ -n "$$($(DOCKER) images | grep $(DOCKER_IMAGE))" ]; then		\
		$(DOCKER) rmi $(DOCKER_IMAGE);					\
	fi

run-devenv:
	$(DOCKER) run --name $(DOCKER_CONTAINER) -i -t $(DOCKER_IMAGE)

start-devenv:
	$(DOCKER) start $(DOCKER_CONTAINER)

stop-devenv:
	$(DOCKER) stop $(DOCKER_CONTAINER)

.PHONY: build-devenv rebuild-devenv remove-devenv run-devenv start-devenv stop-devenv
