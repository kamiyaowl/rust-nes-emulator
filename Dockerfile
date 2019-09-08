FROM rust:1.37.0-buster

RUN apt-get update \
  && apt-get install -y git libgtk-3-dev

RUN mkdir /work
WORKDIR /work

CMD ["/bin/sh"]