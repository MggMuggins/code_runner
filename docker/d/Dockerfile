FROM cereris/dlang:dmd-2.075.0

RUN adduser coder

ADD --chown=coder:coder code /home/coder/code.d

WORKDIR /home/coder

RUN sed -i '1s/^/void main() {\n/' code.d
RUN sed -i '$s/$/\n}/' code.d

USER coder:coder

CMD ["timeout", "-t", "1200", "/bin/sh", "-c", "dmd code.d && /home/coder/code"]

USER root:root
