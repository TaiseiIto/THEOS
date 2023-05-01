# This makefile is called from ../Makefile
# Don't execute it directly.

TMUX=tmux
QEMU_DIR=../.qemu
RUN=run.conf
DEBUG=debug.conf

run:
	$(TMUX) new-session \; source-file $(RUN)

stop:
	make stop -C $(QEMU_DIR)
	for i in $$(ps ax | grep $(TMUX) | grep -v grep | grep -v make | awk '{print $$1}'); do kill $$i; done

debug:
	$(TMUX) new-session \; source-file $(DEBUG)
