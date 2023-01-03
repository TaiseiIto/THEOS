DOCKER = docker
NAME = theos
NO_PREFIX = .no
CONTAINER = .container
CONTAINER_NAME = $(NAME)
NO_CONTAINER = $(NO_PREFIX)$(CONTAINER)
IMAGE = .image
IMAGE_NAME = $(NAME)
NO_IMAGE = $(NO_PREFIX)$(IMAGE)

$(CONTAINER): $(IMAGE)
	$(DOCKER) create --name $(CONTAINER_NAME) -i -t $(IMAGE_NAME)
	rm -f $(NO_CONTAINER)
	touch $@

$(IMAGE): Dockerfile
	$(DOCKER) build --no-cache -t $(IMAGE_NAME) .
	rm -f $(NO_IMAGE)
	touch $@

clean: $(NO_CONTAINER) $(NO_IMAGE)
	
$(NO_CONTAINER): $(CONTAINER)
	$(DOCKER) rm $(CONTAINER_NAME)
	rm -f $<
	touch $@

$(NO_IMAGE): $(IMAGE)
	$(DOCKER) rmi $(IMAGE_NAME)
	rm -f $<
	touch $@

