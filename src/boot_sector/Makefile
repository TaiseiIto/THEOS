DIRECTORIES = $(wildcard */)

all: $(shell git ls-files)
	for directory in $(DIRECTORIES); do \
		make -C $$directory; \
	done;

