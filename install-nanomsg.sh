#!/bin/sh
set -ex

cd /tmp
wget https://github.com/nanomsg/nanomsg/archive/1.0.0.tar.gz -O /tmp/nanomsg-1.0.0.tar.gz
tar xfvz /tmp/nanomsg-1.0.0.tar.gz -C /tmp/nanomsg-1.0.0
mkdir /tmp/nanomsg-1.0.0/build
cd /tmp/nanomsg-1.0.0/build && \
    cmake .. && cmake --build .
    
sudo -C "cd /tmp/nanomsg-1.0.0/build && cmake --build . --target install"

sudo ldconfig
