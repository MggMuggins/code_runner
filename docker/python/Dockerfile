FROM jfloff/alpine-python:3.4

RUN adduser coder -h /home/coder -D

ADD --chown=coder:coder code /home/coder/code.py

WORKDIR /home/coder

USER coder:coder
CMD ["python", "code.py"]

USER root:root
