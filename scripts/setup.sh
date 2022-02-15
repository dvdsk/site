#!/bin/bash

# needs a go toolchain in path
# if GOPATH is in path hugo wil be placed in path too

pushd `pwd`
mkdir -p /tmp/hugo_src
cd /tmp/hugo_src
git clone https://github.com/gohugoio/hugo.git
cd hugo
go install --tags extended
popd
