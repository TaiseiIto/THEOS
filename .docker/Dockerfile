# This is Dockerfile to make development environment.

FROM ubuntu

# A name of branch of THEOS repository.
ARG branch

# Don't ask stdin for anithing to install softwares automatically.
ENV DEBIAN_FRONTEND=noninteractive

# Install softwares.
RUN apt update

# Install basic development systems.
RUN apt install build-essential -y

# Install clang
RUN apt install clang -y
RUN apt install lld -y
RUN apt install lldb -y
RUN apt install llvm -y

# Install data transmission system.
RUN apt install curl -y
RUN apt install wget -y

# Install mkfs.fat
RUN apt install dosfstools -y

# Install mkfs.exfat
RUN apt install exfat-fuse -y
WORKDIR /root
RUN wget http://mirrors.kernel.org/ubuntu/pool/universe/e/exfat-utils/exfat-utils_1.3.0-1_amd64.deb
RUN apt install ./exfat-utils_1.3.0-1_amd64.deb -y
RUN rm exfat-utils_1.3.0-1_amd64.deb
RUN apt install exfatprogs -y

# Install mtools
RUN apt install mtools -y

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
RUN apt install ed -y
RUN apt install vim -y

# Clone this repository.
WORKDIR /root
RUN git clone --single-branch --branch $branch --depth 1 --recursive --shallow-submodules https://github.com/TaiseiIto/THEOS.git

# Build EDK2.
RUN apt install acpica-tools -y
RUN apt install nasm -y
RUN apt install python3 -y
RUN apt install uuid-dev -y
WORKDIR /root/THEOS/qemu/roms/edk2
RUN ./OvmfPkg/build.sh -a X64
RUN rm Conf/target.txt

# Install QEMU.
RUN apt install libglib2.0-dev -y
RUN apt install libpixman-1-dev -y
RUN apt install ninja-build -y
RUN apt install pkg-config -y
WORKDIR /root/THEOS/qemu
RUN ./configure --target-list=x86_64-softmmu CFLAGS="-O0 -g -fno-inline" CXXFLAGS="-O0 -g -fno-inline"
RUN make
RUN make install

# Install GDB.
WORKDIR /root/THEOS/gdb
RUN apt install libgmp-dev -y
RUN apt install dejagnu -y
RUN apt install libexpat-dev -y
RUN apt install iasl -y
RUN apt install libmpfr-dev -y
RUN apt install texinfo -y
RUN apt install bison -y
RUN apt install flex -y
RUN ./configure --with-expat CFLAGS="-O0 -g -fno-inline" CXXFLAGS="-O0 -g -fno-inline"
RUN make
RUN make install
RUN cp /root/THEOS/.gdb/.conf /root/.gdbinit

# Clone MikanOS for reference.
WORKDIR /root
RUN git clone --single-branch --branch master --depth 1 https://github.com/TaiseiIto/mikanos.git
RUN git clone --single-branch --branch master --depth 1 https://github.com/TaiseiIto/mikanos-build.git osbook
WORKDIR /root/osbook
RUN apt install ansible -y
WORKDIR /root/osbool/devenv
RUN ansible-playbook -i ansible_inventory ansible_provision.yml
WORKDIR /root/edk2
RUN ln -s /root/mikanos/MikanLoaderPkg ./

# bash settings.
WORKDIR /root/THEOS
RUN cat .bash/.bashrc >> /root/.bashrc

# tmux settings.
RUN cat .tmux/.tmux.conf >> /root/.tmux.conf

# vim settings.
RUN cat .vim/.vimrc >> /root/.vimrc

# VNC port
EXPOSE 5900

