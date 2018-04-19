FROM mhart/alpine-node:latest

RUN adduser coder -h /home/coder -D

ADD --chown=coder:coder code /home/coder/code.js

WORKDIR /home/coder

USER coder:coder
CMD ["timeout", "-t", "1200", "node", "code.js"]

USER root:root
