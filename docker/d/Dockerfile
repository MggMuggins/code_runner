FROM cereris/dlang:dmd-2.075.0

RUN adduser coder -h /home/coder -D

ADD --chown=coder:coder code /home/coder/code.d

WORKDIR /home/coder

RUN sed -i '1s/^/void main() {\n/' code.d
RUN { cat code.d; echo '}'; } > code.d

USER coder:coder

CMD ["dmd", "code.d"]
CMD ["./code"]

USER root:root
