#!/bin/sh
set -ex

cd /tmp
wget https://github.com/nanomsg/nanomsg/archive/1.0.0.tar.gz -O /tmp/nanomsg-.0.0.tar.gz
tar xfvz /tmp/nanomsg-.0.0.tar.gz
mkdir nanomsg-1.0.0/build && \
  cd nanomsg-1.0.0/build && \
    cmake .. && cmake --build . && \
      sudo cmake --build . --target install
sudo ldconfig
