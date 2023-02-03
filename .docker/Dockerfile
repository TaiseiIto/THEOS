# This is Dockerfile to make development environment.

FROM ubuntu

# Don't ask stdin for anithing to install softwares automatically.
ENV DEBIAN_FRONTEND=noninteractive

# Install softwares.
RUN apt update

# Install basic development systems.
RUN apt install build-essential -y

# Install data transmission system.
RUN apt install curl -y

# Install a version control system.
RUN apt install git -y
RUN apt install git-email -y
RUN git config --global pull.rebase false
RUN mkdir /root/.ssh
RUN mkdir /root/.github

# Install sponge
RUN apt install moreutils -y

# Install Rust.
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y
RUN mkdir /root/.crates.io

# Install a terminal multiplexer.
RUN apt install tmux -y

# Set timezone to Japan.
RUN apt install tzdata -y
RUN cp /usr/share/zoneinfo/Asia/Tokyo /etc/localtime

# Install an editor.
RUN apt install vim -y

# Build EDK2
RUN apt install acpica-tools -y
RUN apt install nasm -y
RUN apt install python3 -y
RUN apt install uuid-dev -y
WORKDIR /root
RUN git clone --recursive https://github.com/tianocore/edk2.git
WORKDIR /root/edk2
RUN ./OvmfPkg/build.sh -a X64

# Install qemu.
RUN apt install libglib2.0-dev -y
RUN apt install libpixman-1-dev -y
RUN apt install ninja-build -y
RUN apt install pkg-config -y
WORKDIR /root
RUN git clone https://gitlab.com/qemu-project/qemu.git
WORKDIR /root/qemu
RUN ./config --target-list=x86_64-softmmu CFLAGS="-O0 -g -fno-inline" CXXFLAGS="-O0 -g -fno-inline"
RUN make
RUN make install

# Clone this repository.
WORKDIR /root
RUN git clone https://github.com/TaiseiIto/THEOS.git
WORKDIR THEOS

# bash settings.
RUN cat .bash/.bashrc >> /root/.bashrc

# tmux settings.
RUN cat .tmux/.tmux.conf >> /root/.tmux.conf

# vim settings.
RUN cat .vim/.vimrc >> /root/.vimrc

# VNC port
EXPOSE 5900

