FROM node:10-buster AS node-build

# tzdata for building
ENV TZ=Etc/UTC
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN apt-get update && apt-get -y install build-essential \
  cmake \
  pkg-config \
  libgazebo9 libgazebo9-dev \
  libjansson-dev \
  imagemagick

# Set Gazebo related env
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
WORKDIR /opt
RUN git clone https://github.com/osrf/gazebo_models -b master
WORKDIR /opt/gazebo_models
RUN mkdir build
WORKDIR /opt/gazebo_models/build
RUN cmake .. -DCMAKE_INSTALL_PREFIX=/opt/gzweb/http/client && make install > /dev/null 2>&1
RUN mv /opt/gzweb/http/client/models /opt/gzweb/http/client/assets
WORKDIR /opt/gzweb
RUN rm -rf /opt/gazebo_models

RUN ./get_local_models.py /opt/gzweb/http/client/assets
RUN ./webify_models_v2.py /opt/gzweb/http/client/assets

# Declared once more to make sure we didn't forget to toggle back here at this stage
WORKDIR /opt/gzweb

EXPOSE 8080

CMD ["npm", "run", "start"]
