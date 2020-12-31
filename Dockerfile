FROM node:10-buster AS node-build

ENV TZ=Etc/UTC
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN apt-get update && apt-get -y install build-essential \
  cmake \
  pkg-config \
  libgazebo9 libgazebo9-dev \
  libjansson-dev \
  imagemagick

COPY . /opt/gzweb/.
WORKDIR /opt/gzweb

RUN npm install
RUN /opt/gzweb/node_modules/.bin/grunt build

RUN rm -rf build && mkdir build
WORKDIR /opt/gzweb/build
RUN cmake ..
RUN make -j 8

WORKDIR /opt/gzweb/gzbridge
RUN /opt/gzweb/node_modules/.bin/node-gyp rebuild -d

# TODO Model build

WORKDIR /opt/gzweb

EXPOSE 8080

CMD ["npm", "run", "start"]
