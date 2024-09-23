FROM ubuntu:latest
LABEL authors="sten"

ENTRYPOINT ["top", "-b"]