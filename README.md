# cls-linux-rs
## 快速安装
### x86

```bash
wget https://mirror.ghproxy.com/https://github.com/taikulawo/cls/releases/download/release/cls-x86_64-unknown-linux-musl.tar.gz -O /tmp/cls.tar.gz
sudo tar xvzf /tmp/cls.tar.gz -C /usr/local/bin
```
### arm64

```bash
wget https://mirror.ghproxy.com/https://github.com/taikulawo/cls/releases/download/release/cls-aarch64-unknown-linux-musl.tar.gz -O /tmp/cls.tar.gz
sudo tar xvzf /tmp/cls.tar.gz -C /usr/local/bin
```

或者到 cls 主页点击 install.sh 的Raw，GitHub会给你一个加token的url，替换掉下面的token

```bash
bash <(curl -sSL https://raw.githubusercontent.com/taikulawo/cls/master/install.sh?token=<token>)
```

### Cargo编译安装

```bash
cargo install --git https://github.com/taikulawo/cls
```

## 查看实时日志

```bash
journalctl -u cls-linux -f
# 优先展示最新的日志
# 注意：-f --reverse冲突
# journalctl -u cls-linux --reverse
```

## FAQ && 注意事项
1. cls配置文件
   - rules只加根据大陆IP分流，和内网CIDR分流。其他的都不加
   - why: dl.google.com 在[一些rule](https://raw.githubusercontent.com/Loyalsoldier/clash-rules/release/direct.txt)上是DIRECT，实际通不了
2. 订阅链接删除非proxy（咖啡猫的订阅返回剩余流量的profile，）

3. 某些公司禁止用外网DNS解析，代理是域名会导致解析失败，不可用
   解决办法：在clash for windows 的 dns 加上内网 DNS ip
   ![image](https://github.com/iamwwcposts/articles/assets/24750337/6dcde701-10b0-47c6-83b4-469452fdf76e)

   surge加上 `dns-server: system`
   奇怪的一点，开启tun后，cfw mac的system dns会被改成 8.8.8.8，而不是保留DHCP分配的dns。会导致一部分访问失败（dns-hijack没拦住）
4. 时间不准确会导致clash不可用
vmess 依赖系统时间。

时间同步
```bash
apt install systemd-timesyncd -y
```

## Server

提供在线配置转换，将不同订阅商的配置统一成适合我用的配置
编译安装

```bash
make
sudo make install
```

树莓派公网转换服务

```
https://api.chaochaogege.com/cls-converter?url=?
```

## generate

```bash
cat data/config.yaml | cargo run --bin cls -- generate --tun > test.yaml
```
