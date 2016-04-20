FROM alpine:3.3
MAINTAINER Silas Rech "lenovouser@minora.io"

RUN apk update && apk add file sudo curl git && \
    rm -rf /var/cache/apk/* && \
    curl -sSf https://static.rust-lang.org/rustup.sh | sh && \
    mkdir /build/ && \
    git clone https://github.com/minora-oss/bolt /build/bolt/ && \
    cd /build/bolt/ && \
    cargo install && \
    bash /usr/local/lib/rustlib/uninstall.sh && \
    rm -rf /build/ && \
    rm -rf /etc/rust* && \
    rm -rf /root/.rust* && \
    rm -rf /root/rust* && \
    rm -rf /usr/local/lib/rust* && \
    rm -rf /usr/local/bin/rust* && \
    rm -rf /usr/local/bin/cargo* && \
    apk del file sudo curl git && \
    rm -rf /var/cache/apk/*

VOLUME [ "/bolt/" ]
WORKDIR "/bolt/"

ENTRYPOINT [ "/usr/bin/bolt" ]
