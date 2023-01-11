SOURCE = $(shell ls *.c)
OBJECT = $(SOURCE:.c=.o)
ARCHIVE = $(OBJECT:.o=.a)

all: $(ARCHIVE)

$(ARCHIVE): $(OBJECT)
	ar -r $@ $^

$(OBJECT): $(SOURCE)
	gcc $^ -fPIC -c -o $@

