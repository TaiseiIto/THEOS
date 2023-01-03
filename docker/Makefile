ifeq ($(OS), Windows_NT)
CLEAR = del
DONE = copy nul
else
CLEAR = rm -f
DONE = touch
endif

DOCKER = docker
NAME = theos
NO_PREFIX = .no
RUN_PREFIX = .run
CONTAINER = .container
CONTAINER_NAME = $(NAME)
NO_CONTAINER = $(NO_PREFIX)$(CONTAINER)
RUN_CONTAINER = $(RUN_PREFIX)$(CONTAINER)
IMAGE = .image
IMAGE_NAME = $(NAME)
NO_IMAGE = $(NO_PREFIX)$(IMAGE)

all: $(RUN_CONTAINER)
	$(DOCKER) attach $(CONTAINER_NAME)
	$(CLEAR) $(RUN_CONTAINER)

clean: $(NO_CONTAINER) $(NO_IMAGE)

rebuild: clean
	make

empowerment: $(CONTAINER)
	$(DOCKER) cp $(GITHUB) $(CONTAINER_NAME):/root/.github/key
	$(DOCKER) cp $(GITGPG) $(CONTAINER_NAME):/root/.gnupg
	$(DOCKER) cp $(CRATES) $(CONTAINER_NAME):/root/.crates.io/key
	$(DOCKER) start $(CONTAINER_NAME)
	$(DOCKER) exec -i -t $(CONTAINER_NAME) /root/THEOS/git/gitconfig.sh
	$(DOCKER) stop $(CONTAINER_NAME)

$(RUN_CONTAINER): $(CONTAINER)
	$(DOCKER) start $(CONTAINER_NAME)
	
$(CONTAINER): $(IMAGE)
	$(DOCKER) create --name $(CONTAINER_NAME) -i -t $(IMAGE_NAME) /bin/bash
	$(CLEAR) $(NO_CONTAINER)
	$(DONE) $@

$(IMAGE): Dockerfile
	$(DOCKER) build --no-cache -t $(IMAGE_NAME) .
	$(CLEAR) $(NO_IMAGE)
	$(DONE) $@

$(NO_CONTAINER):
	$(DOCKER) rm -f $(CONTAINER_NAME)
	$(CLEAR) $(CONTAINER)
	$(DONE) $@

$(NO_IMAGE):
	$(DOCKER) rmi -f $(IMAGE_NAME)
	$(CLEAR) $(IMAGE)
	$(DONE) $@

