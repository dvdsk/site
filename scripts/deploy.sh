#!/usr/bin/env bash
set -e

BUILD_ARG=$1
SERVER="sgc"  # ssh config name or full adress

RELEASE=debug
if [[ $BUILD_ARG == "--release" ]]; then
	RELEASE=release
fi

# update hugo site
(cd public && hugo -D)
rsync -r public/public sgc:/tmp/
rsync -r forwarded.txt sgc:/tmp/


cross build --target=aarch64-unknown-linux-gnu $BUILD_ARG
rsync -vh --progress \
  target/aarch64-unknown-linux-gnu/$RELEASE/webserver \
  $SERVER:/tmp/

cmds="
sudo rm -rf /home/webserver/files /home/webserver/webserver
sudo mkdir /home/webserver/files
sudo mv /tmp/public /home/webserver/files/
sudo mv /tmp/forwarded.txt /home/webserver/forwarded.txt
sudo mv /tmp/webserver /home/webserver/webserver
sudo chown -R webserver:webserver /home/webserver/
sudo systemctl restart webserver.service
"

ssh -t sgc "$cmds"
