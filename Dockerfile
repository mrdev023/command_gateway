FROM ubuntu:latest

# Configure requirements
RUN apt update && apt install openssh-server curl build-essential protobuf-compiler -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN useradd -rm -d /home/test -s /opt/gateway/target/release/client test
RUN echo 'test:test' | chpasswd

# Build project binaries
COPY . /opt/gateway
WORKDIR /opt/gateway
RUN cargo build --release --bin daemon
RUN cargo build --release --bin client
RUN mkdir -p /var/run
RUN chmod ugo+rx /var
RUN chmod ugo+rwx /var/run

RUN echo '#!/bin/sh\n\
/usr/sbin/sshd -D&\n\
su - test -s /bin/sh -c "/opt/gateway/target/release/daemon"' > /opt/startup.sh

RUN chmod +x /opt/startup.sh

RUN service ssh start

EXPOSE 22

CMD /opt/startup.sh
