# This makefile is called from ../Makefile
# Don't execute it directly.

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
VNC_PORT = 5900

# Enter development environment.
all: $(RUN_CONTAINER)
	$(CLEAR) $(RUN_CONTAINER)
	$(DOCKER) attach $(CONTAINER_NAME)

# Clean development environment.
clean: $(NO_CONTAINER) $(NO_IMAGE)

# Rebuild development environment.
rebuild: clean
	make

# Get development permission.
permission: $(CONTAINER)
	$(DOCKER) cp $(GITHUB) $(CONTAINER_NAME):/root/.github/key
	$(DOCKER) cp $(GITGPG) $(CONTAINER_NAME):/root/.gnupg
	$(DOCKER) cp $(CRATESIO) $(CONTAINER_NAME):/root/.crates.io/key
	$(DOCKER) start $(CONTAINER_NAME)
	$(DOCKER) exec -i -t $(CONTAINER_NAME) /root/THEOS/.git.conf/gitconfig.sh
	$(DOCKER) stop $(CONTAINER_NAME)

# Start a Docker container.
$(RUN_CONTAINER): $(CONTAINER)
	$(DOCKER) start $(CONTAINER_NAME)
	$(DONE) $(RUN_CONTAINER)
	
# Make a Docker container.
$(CONTAINER): $(IMAGE)
	$(DOCKER) create --name $(CONTAINER_NAME) -p $(VNC_PORT):$(VNC_PORT) --privileged -i -t $(IMAGE_NAME) /bin/bash
	$(CLEAR) $(NO_CONTAINER)
	$(DONE) $@

# Make a Docker image.
$(IMAGE): Dockerfile
	$(DOCKER) build --no-cache -t $(IMAGE_NAME) .
	$(CLEAR) $(NO_IMAGE)
	$(DONE) $@

# Delete a Docker container.
$(NO_CONTAINER):
	$(DOCKER) rm -f $(CONTAINER_NAME)
	$(CLEAR) $(CONTAINER)
	$(DONE) $@

# Delete a Docker image.
$(NO_IMAGE):
	$(DOCKER) rmi -f $(IMAGE_NAME)
	$(CLEAR) $(IMAGE)
	$(DONE) $@

