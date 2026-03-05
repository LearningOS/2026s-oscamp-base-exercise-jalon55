FROM ubuntu:24.04

# 设置环境变量
ENV LANG C.UTF-8
ENV LANGUAGE C.UTF-8
ENV DEBIAN_FRONTEND noninteractive

# 安装基础工具和依赖
RUN apt-get update && apt-get install -y \
    git wget unzip openssh-server curl \
    jq gcc-riscv64-linux-gnu qemu-user-static \
    build-essential libssl-dev zlib1g-dev \
    libbz2-dev libreadline-dev libsqlite3-dev \
    wget curl llvm libncurses5-dev libncursesw5-dev \
    xz-utils tk-dev libffi-dev liblzma-dev \
    zsh

# 安装 Rust 工具链并添加 riscv64 目标
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    /root/.cargo/bin/rustup target add riscv64gc-unknown-linux-gnu

# 设置 PATH 环境变量
ENV PATH="/root/.cargo/bin:$PATH"

# 安装 code-server 和插件
RUN curl -fsSL https://code-server.dev/install.sh | sh && \
    code-server --install-extension cnbcool.cnb-welcome && \
    code-server --install-extension redhat.vscode-yaml && \
    code-server --install-extension waderyan.gitblame && \
    code-server --install-extension mhutchie.git-graph && \
    code-server --install-extension donjayamanne.githistory && \
    code-server --install-extension cloudstudio.live-server && \
    code-server --install-extension tencent-cloud.coding-copilot && \
    code-server --install-extension rust-lang.rust-analyzer

# 安装 Oh My Zsh
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" && \
    git clone https://github.com/zsh-users/zsh-autosuggestions ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-autosuggestions && \
    git clone https://github.com/zsh-users/zsh-syntax-highlighting.git ${ZSH_CUSTOM:-~/.oh-my-zsh/custom}/plugins/zsh-syntax-highlighting

# 配置 Zsh 作为默认 shell
RUN chsh -s /usr/bin/zsh

# 清理包缓存
RUN apt-get clean && rm -rf /var/lib/apt/lists/*

CMD ["zsh"]