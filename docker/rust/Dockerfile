FROM frolvlad/alpine-rust:latest

RUN adduser coder -h /home/coder -D

ADD --chown=coder:coder code /home/coder/code.rs

WORKDIR /home/coder

RUN sed -i '1s/^/fn main() {\n/' code.rs
RUN sed -i '$s/$/\n}/' code.rs

USER coder:coder

CMD rustc code.rs && /home/coder/code

USER root:root
