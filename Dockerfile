FROM ubuntu:17.10 as musl-cross
RUN apt-get update
RUN apt-get install -y git build-essential curl wget
ENV HOME=/root
ENV USER=root
WORKDIR /root
RUN git clone https://github.com/richfelker/musl-cross-make.git
WORKDIR /root/musl-cross-make
RUN echo 'sources: $(SRC_DIRS)' >> Makefile
RUN make sources -j4
RUN make TARGET=armv7-linux-musleabihf -j4
RUN make TARGET=armv7-linux-musleabihf install
RUN tar -cf armv7-linux-musleabihf.tar output

FROM ubuntu:17.10 as qemu
RUN apt-get update
RUN apt-get install -y qemu-user-static curl
ENV HOME=/root
ENV USER=root
WORKDIR /root
RUN curl https://sh.rustup.rs -sSf > rustup.sh
RUN sh rustup.sh -y
ENV PATH=/root/output/bin:/root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
RUN rustup target add armv7-unknown-linux-musleabihf
COPY --from=musl-cross /root/musl-cross-make/armv7-linux-musleabihf.tar armv7-linux-musleabihf.tar
RUN tar -xf armv7-linux-musleabihf.tar
RUN cargo init --bin
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
RUN cargo build --release --target=armv7-unknown-linux-musleabihf
COPY . ./
RUN cargo build --release --target=armv7-unknown-linux-musleabihf

# FROM arm32v6/alpine
# COPY --from=qemu /usr/bin/qemu-arm-static /usr/bin/
# RUN apk update
# RUN apk upgrade
# RUN apk add curl
# RUN apk add libc6-compat
# RUN apk add libgcc
# RUN sh rustup.sh -y
# # armv7-unknown-linux-musleabihf
