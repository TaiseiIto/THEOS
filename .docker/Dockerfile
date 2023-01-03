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

# Install Rust.
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y
RUN mkdir /root/.crates.io

# Install a terminal multiplexer.
RUN apt install tmux -y

# Install an editor.
RUN apt install vim -y

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

