# この Dockerfile は Docker デーモンが読んで
# このファイルに基づいてイメージを構築してくれる


# 基礎となるイメージを指定する
FROM mcr.microsoft.com/devcontainers/base:jammy

# qtbase5-dev と libopencv-dev をインストールする
# これで OpenCV が導入できる
RUN sudo apt update; \
    sudo apt -y install qtbase5-dev libopencv-dev

USER vscode
RUN curl https://sh.rustup.rs/ -sSf | sh -s -- -y

RUN sudo apt -y install libclang-dev
RUN sudo apt -y install clang

RUN sudo groupadd -g 985 video-2 && \
    sudo usermod -a -G video-2 vscode