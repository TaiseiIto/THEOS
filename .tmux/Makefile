# This makefile is called from ../Makefile
# Don't execute it directly.

TMUX=tmux
QEMU_DIR=../.qemu
RUN_QEMU=run_qemu.conf

run_qemu:
	-$(TMUX) new-session \; source-file $(RUN_QEMU)

stop_qemu:
	make stop -C $(QEMU_DIR)
	for i in $$(ps ax | grep $(TMUX) | grep -v grep | awk '{print $$1}'); do kill $$i; done

