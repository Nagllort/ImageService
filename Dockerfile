FROM rust:1.41

RUN groupadd -r image-service && useradd -r -g image-service image-service
WORKDIR /image-service
COPY ./image-service /image-service
COPY ./run.sh /
USER image-service
CMD ["/run.sh"]

