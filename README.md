# THEOS

An OS written in [Rust](https://www.rust-lang.org/) and working on x64 architecture and UEFI.

This is developed in reference to [mikanos](https://github.com/uchan-nos/mikanos) by [uchan-nos](https://github.com/uchan-nos).

Pronunciation of the name is \[t&#x2B0;e.&#xF3;s\] in [IPA](https://en.wikipedia.org/wiki/International_Phonetic_Alphabet).

## Development environment

THEOS is developed on Docker container provided by [`.docker`](.docker) directory.

Make Docker image and container and enter it.

```
/somewhere $ git clone https://github.com/TaiseiIto/THEOS.git
/somewhere $ cd THEOS
/somewhere/THEOS $ make docker
~/THEOS #
```

Now you are in the development environment!

## Run THEOS on QEMU

In the development environment, Run THEOS on QEMU by the command bellow.

```
~/THEOS # make run
```

[tmux](https://github.com/tmux/tmux) splits a screen left and right.
The left one is THEOS serial console.
The right one is terminal of the development environment.
You can move the left pane by `Ctrl+t` and `h`, or the right pane by `Ctrl+t` and `l`.
And you can use [VNC viewer](https://www.realvnc.com/en/connect/download/viewer/) on host and connect to `localhost:5900` and operate THEOS running on QEMU on the development environment.

## Stop THEOS on QEMU

Move the right pane by `Ctrl+t` and `l` and stop THEOS by the command bellow.

```
~/THEOS # make stop
```

## Run on GPD MicroPC 2021ver

I use [GPD MicroPC 2021ver](https://gpd-direct.jp/pages/gpd-micropc) to check the operation for THEOS.

### Prepare THEOS bootable device

Prepare a USB memory and format it in FAT file system.
When you `make` THEOS, there is `root` directory in the repository.

```
~/THEOS # make
~/THEOS # ls
LICENSE  Makefile  README.md  imager  imager.log  qemu  root  src  theos.img
```

This `root` directory should be root directory of the USB memory.
Copy files and subdirectories of the `root` directory into the USB memory.
Eject the USB memory and insert into GPD MicroPC.

### Set BIOS and power on

Power on GPD MicroPC and push ESC key repeatedly.
Set as in the example below.

* Security
	* Secure Boot
		* Secure Boot: [Disable]
* Boot
	* Quiet Boot: [Disable]
	* Fast Boot: [Disable]
	* FIXED BOOT ORDER Priorities
		* Boot Option #1: [USB Key:UEFI: USB DISK 2.0 PMAP]
	* UEFI Hard Disk Drive BBS Priorities
		* Boot Option #1: [UEFI OS (P0: BIWIN SSD)]

Save and exit.
Then, GPD MicroPC restarts and THEOS starts.

### Power off and reset BIOS

Power on GPD MicroPC and push ESC key repeatedly.
Set as in the example below.

* Security
	* Secure Boot
		* Secure Boot: [Disable]
* Boot
	* Quiet Boot: [Enable]
	* Fast Boot: [Enable]
	* FIXED BOOT ORDER Priorities
		* Boot Option #1: [Hard Disk:Windows Boot Manager (P0: BIWIN SSD)]
	* UEFI Hard Disk Drive BBS Priorities
		* Boot Option #1: [Windows Boot Manager (P0: BIWIN SSD)]

Save and exit.
Then, GPD MicroPC restarts and Windows starts.

## Get development permission (for only developers, not users)

To get development permission, you need to prepare below.

* SSH key to push to [the remote repository](https://github.com/TaiseiIto/THEOS).
* .gnupg directory to verify your commitments.
* API key to log in [`crates.io`](https://crates.io/).

And `make permission` as below.

```
~/THEOS # exit
/somewhere/THEOS $ make permission GITHUB=/path/to/ssh/key GITGPG=/path/to/.gnupg CRATESIO=/path/to/crates.io/API/key
Your GitHub user name: Someone
Your Github email address: someone@example.com
Password for someone@example.com: *******
/somewhere/THEOS $
```

Now, you have development permission.

