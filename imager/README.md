# imager

In this directory, `imager`, a disk image generator, is developed.

`imager` is automatically called by [`../Makefile`](../Makefile) and generates exFAT raw disk image of THEOS boot drive.

So, you don'd have to execute it directly.

## Usage

```
$ ./target/release/imager /path/to/boot/sector /path/to/source/directory /path/to/destination
```

`/path/to/boot/sector` specifies a raw image file of the boot sector of the THEOS boot drive.

`/path/to/source/directory` specifies source directory.

The boot drive contains a directory tree whose root is specified source directory.

`imager` generates an image file and put it in `/path/to/destination`.

