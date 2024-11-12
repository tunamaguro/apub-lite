FROM rust:1.82.0-slim-bookworm as dev

RUN apt-get update -y && \
    apt-get install -y \
    git 

ARG USERNAME=vscode
ARG GROUPNAME=vscode
ARG UID=1000
ARG GID=1000
RUN groupadd -g $GID $GROUPNAME && \
    useradd -m -s /bin/bash -u $UID -g $GID $USERNAME

USER ${USERNAME}

RUN rustup component add rustfmt clippy

# Enable completions
RUN echo "source /usr/share/bash-completion/completions/git" >> ~/.bashrc
RUN echo "source <( rustup completions bash )" >> ~/.bashrc
RUN echo "source <( rustup completions bash cargo )" >> ~/.bashrc

# Install tools
RUN cargo install just