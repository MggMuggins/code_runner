FROM gendosu/alpine-ruby:latest

RUN adduser coder -h /home/coder -D

ADD --chown=coder:coder code /home/coder/code.rb

WORKDIR /home/coder

USER coder:coder
CMD ["timeout", "20m", "ruby", "code.rb"]

USER root:root
