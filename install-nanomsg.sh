#!/bin/sh
set -ex

wget https://github.com/nanomsg/nanomsg/archive/1.0.0.tar.gz -O /tmp/nanomsg-.0.0.tar.gz
tar xfvz /tmp/nanomsg-.0.0.tar.gz
cd nanomsg-1.0.0 && cmake && sudo cmake --build . --target install
sudo ldconfig
