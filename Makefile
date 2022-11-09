INSTALL = install
PHONE := all
all: build
PHONE += build
build:
	cargo b --bin server --release
PHONE += clean
clean:
	rm -f /lib/systemd/cls-server.service
	rm -f /usr/bin/cls-server
PHONE += install
install:
	$(INSTALL) -m 644 scripts/cls-server.service /lib/systemd/system/cls-server.service
	$(INSTALL) -m 755 target/release/server /usr/bin/cls-server
	systemctl daemon-reload
.PHONE: $(PHONE)