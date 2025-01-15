#!/usr/bin/env bash
set -xe
git submodule update --remote --recursive --init
# setup
# install golang
sudo apt update
sudo apt install libssl-dev pkg-config musl-tools clang jq -y
cargo install cross --git https://github.com/cross-rs/cross

# install docker
# Add Docker's official GPG key:
sudo apt-get update
sudo apt-get install ca-certificates curl gnupg
sudo install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
sudo chmod a+r /etc/apt/keyrings/docker.gpg

# Add the repository to Apt sources:
echo \
"deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
"$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | \
sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt-get update
sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin -y

# 更新代理文件
curl -sL https://cdn.jsdelivr.net/gh/Loyalsoldier/geoip@release/Country-only-cn-private.mmdb > data/Country.mmdb
curl -sL --retry 5 --max-time 10 --retry-delay 2 $(jq -rM .subscribe_url config.json) -o data/config.yaml

# 下载ruleset
# get latest release
# id=$(curl -sL "https://api.github.com/repos/Loyalsoldier/clash-rules/releases" | jq -r '.[0].id')
# result=$(curl -sL -H "Accept: application/vnd.github+json" https://api.github.com/repos/Loyalsoldier/clash-rules/releases/$id/assets)
# len=$(echo $result | jq length)
# [ ! -d data/ruleset ] && mkdir data/ruleset
# for ((i=0; i<$len; i++)) do
#   download_url=$(echo $result | jq -r '.[$i].browser_download_url' --argjson i $i)
#   name=$(echo $result | jq -r '.[$i].name' --argjson i $i)
#   curl -sL $download_url > data/ruleset/$name
# done

# 下载clash dashboard
wget https://github.com/yetpocket/cls-dashboard/releases/download/master/dist.tar.gz -O dist.tar.gz
mkdir -p data/ui
tar xvzf dist.tar.gz -C data/ui
# 下载 Clash.Meta
wget https://github.com/yetpocket/cls-meta/releases/download/latest/clash.meta-$platform-$arch.gz -O cls.gz
gzip -d cls.gz
chmod 755 cls
mv cls data/cls
