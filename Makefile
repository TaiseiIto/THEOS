DOCKER = docker
DOCKER_IMAGE = theos-devenv
DOCKER_IMAGE_TAG = latest
DOCKER_CONTAINER = theos-devenv
DOCKER_SHELL = /bin/sh
GENIMG = genimg/genimg

$(GENIMG):
	make -C $(dir $@)

build-devenv:
	$(DOCKER) build --no-cache -t $(DOCKER_IMAGE):$(DOCKER_IMAGE_TAG) .

clean:
	make clean -C $(dir $(GENIMG))

login-devenv:
	$(DOCKER) attach $(DOCKER_CONTAINER)

rebuild: clean
	make

rebuild-devenv: remove-devenv
	make build-devenv

remove-devenv:
	./script/remove-devenv.sh $(DOCKER) $(DOCKER_IMAGE) $(DOCKER_CONTAINER)

run-devenv:
	$(DOCKER) run --name $(DOCKER_CONTAINER) -i -t $(DOCKER_IMAGE)

start-devenv:
	$(DOCKER) start $(DOCKER_CONTAINER)

stop-devenv:
	$(DOCKER) stop $(DOCKER_CONTAINER)

.PHONY: build-devenv clean rebuild-devenv remove-devenv run-devenv start-devenv stop-devenv
