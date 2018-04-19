FROM alpine

RUN adduser coder -h /home/coder -D

ADD --chown=coder:coder code /home/coder/code.sh

WORKDIR /home/coder
USER coder:coder

CMD ["timeout", "20m", "sh", "code.sh"]

USER root:root
