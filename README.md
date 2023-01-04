# THEOS

An OS written in [Rust](https://www.rust-lang.org/) and working on x64 architecture and UEFI.

This is developed in reference to [mikanos](https://github.com/uchan-nos/mikanos) by [uchan-nos](https://github.com/uchan-nos).

Pronunciation of the name is \[t&#x2B0;e.&#xF3;s\] in [IPA](https://en.wikipedia.org/wiki/International_Phonetic_Alphabet).

## Development environment

THEOS is developed on Docker container provided by [.docker](.docker) directory.

You can make Docker image and container and enter it.

```
/somewhere $ git clone https://github.com/TaiseiIto/THEOS.git
/somewhere $ cd THEOS
/somewhere/THEOS $ make docker
~/THEOS #
```

Now you are in the development environment!

## Get development permission (for only developers, not users)

To get development permission, you need to prepare below.

* SSH key to push to [remote repository](http://github.com/TaiseiIto/THEOS).
* .gnupg directory to verify your commitments.
* API key to log in [crates.io](https://crates.io/).

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

