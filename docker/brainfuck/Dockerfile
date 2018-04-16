FROM mahtaran/alpine-brainfuck:latest

RUN adduser coder -h /home/coder -D

ADD --chown=coder:coder code /home/coder/code.bf

WORKDIR /home/coder

USER coder:coder
CMD ["brainfuck", "code.bf"]

USER root:root
