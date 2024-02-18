# many network need proxy . host run docker cmd need proxy .
# inside the docker container need proxy.
# need to set for this two env.
# or. use US azure server

####    1.set up env:
# Use ubuntu 2204 lts as the base image ,jammy is ubuntu 2204 lts
FROM ubuntu:jammy
# Copy the script file to the container
COPY ./setup.sh ~/setup.sh
# WORKDIR ~/taxlint
# Make the script file executable
# RUN chmod +x ./setup.sh

#### 3.build the image and run
# docker build -t taxlint:v1 . \
# --build-arg "HTTP_PROXY=http://192.168.235.1:25526/" \
#     --build-arg "HTTPS_PROXY=http://192.168.235.1:25526/" \
# need proxy sometimes:
# docker build --build-arg http_proxy="http://127.0.0.53:25526" --build-arg https_proxy="http://127.0.0.53:25526" -t taxlint:v1 .
# docker build --build-arg http_proxy="http://127.0.0.1:25526" --build-arg https_proxy="http://127.0.0.1:25526" -t taxlint:v1 .
# docker build --build-arg http_proxy="http://192.168.235.1:25526" --build-arg https_proxy="http://192.168.235.1:25526" -t taxlint:v1 .
# docker run -it taxlint:v1

#### 4.push image to docker-hub
# docker login ...
# docker tag taxlint:v1 btwl/taxlint:v1

#### 5.user use :
# docker run  btwl/taxlint:v1
