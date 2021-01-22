FROM gazebo:gzserver11 AS model_builder

# tzdata for building
ENV TZ=Etc/UTC
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# Set Gazebo related env
ENV ROS_MASTER_URI=http://localhost:11311
ENV GAZEBO_MASTER_URI=http://localhost:11345
ENV GAZEBO_MODEL_DATABASE_URI=http://gazebosim.org/models
ENV GAZEBO_RESOURCE_PATH=/usr/share/gazebo-9:${GAZEBO_RESOURCE_PATH}
ENV GAZEBO_PLUGIN_PATH=/usr/lib/x86_64-linux-gnu/gazebo-9/plugins:${GAZEBO_PLUGIN_PATH}
ENV GAZEBO_MODEL_PATH=/usr/share/gazebo-9/models:${GAZEBO_MODEL_PATH}
ENV LD_LIBRARY_PATH=${LD_LIBRARY_PATH}:/usr/lib/x86_64-linux-gnu/gazebo-9/plugins
ENV OGRE_RESOURCE_PATH=/usr/lib/x86_64-linux-gnu/OGRE-1.9.0

# Dependencies
RUN apt-get update && apt-get -y install build-essential \
  git \
  cmake

# Build a local model database
WORKDIR /opt
RUN mkdir -p /opt/gzweb/http/client
RUN git clone https://github.com/osrf/gazebo_models -b master
WORKDIR /opt/gazebo_models
RUN mkdir build
WORKDIR /opt/gazebo_models/build
RUN cmake .. -DCMAKE_INSTALL_PREFIX=/opt/gzweb/http/client && make install > /dev/null 2>&1

# TODO Turtlebot3 simulation build

FROM ubuntu:focal AS overmind_builder
RUN apt-get update && apt-get -y install git

RUN mkdir /opt/overmind
WORKDIR /opt/overmind
ADD https://github.com/DarthSim/overmind/releases/download/v2.2.0/overmind-v2.2.0-linux-386.gz /opt/overmind/overmind.gz
RUN gunzip overmind.gz

FROM ros:noetic AS belvedere

# tzdata for building
ENV TZ=Etc/UTC
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN apt-get update && apt-get -y install build-essential \
  cmake \
  pkg-config \
  gazebo9 libgazebo9 libgazebo9-dev \
  libjansson-dev \
  tmux \
  curl \
  git \
  vim \
  python \
  imagemagick

# Set Gazebo related env
ENV ROS_MASTER_URI=http://localhost:11311
ENV GAZEBO_MASTER_URI=http://localhost:11345
ENV GAZEBO_MODEL_DATABASE_URI=http://gazebosim.org/models
ENV GAZEBO_RESOURCE_PATH=/usr/share/gazebo-9:${GAZEBO_RESOURCE_PATH}
ENV GAZEBO_PLUGIN_PATH=/usr/lib/x86_64-linux-gnu/gazebo-9/plugins:${GAZEBO_PLUGIN_PATH}
ENV GAZEBO_MODEL_PATH=/usr/share/gazebo-9/models:${GAZEBO_MODEL_PATH}
ENV LD_LIBRARY_PATH=${LD_LIBRARY_PATH}:/usr/lib/x86_64-linux-gnu/gazebo-9/plugins
ENV OGRE_RESOURCE_PATH=/usr/lib/x86_64-linux-gnu/OGRE-1.9.0

COPY . /opt/gzweb/.
WORKDIR /opt/gzweb

# Build JS dependencies
RUN mkdir /opt/n
WORKDIR /opt/n
RUN curl -L https://raw.githubusercontent.com/tj/n/master/bin/n -o n
RUN mv n /usr/local/bin/n
RUN chmod +x /usr/local/bin/n
RUN n 10
WORKDIR /opt/gzweb
RUN npm install
RUN /opt/gzweb/node_modules/.bin/grunt build

# Build C and Python dependencies
RUN rm -rf build && mkdir build
WORKDIR /opt/gzweb/build
RUN cmake ..
RUN make -j 8

# Build gzbridge
WORKDIR /opt/gzweb/gzbridge
RUN /opt/gzweb/node_modules/.bin/node-gyp rebuild -d

# Build a local model database
COPY --from=model_builder /opt/gzweb/http/client/models /opt/gzweb/http/client/assets
WORKDIR /opt/gzweb

RUN python get_local_models.py /opt/gzweb/http/client/assets
RUN python webify_models_v2.py /opt/gzweb/http/client/assets

# Generate thumbnails
# RUN bash tools/gzthumbnails.sh

# Set up Overmind
COPY --from=overmind_builder /opt/overmind/overmind /usr/local/bin/overmind
RUN chmod +x /usr/local/bin/overmind

# Declared once more to make sure we didn't forget to toggle back here at this stage
WORKDIR /opt/gzweb

EXPOSE 8080

CMD ["overmind", "start", "-N"]
