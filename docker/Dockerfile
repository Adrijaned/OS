FROM debian:latest

ENV PATH "$PATH:/root/.cargo/bin"
RUN apt-get update &&\
	apt-get install -y wget gcc nasm make git genisoimage netcat &&\
	wget -q "https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init" &&\
	chmod +x rustup-init &&\
	./rustup-init -y --default-toolchain nightly &&\
	rm rustup-init &&\
	$HOME/.cargo/bin/rustup component add rust-src &&\
	$HOME/.cargo/bin/cargo install xargo &&\
	git clone "https://github.com/Adrijaned/OS.git" &&\
	cd OS && make && make clean && cd ..
EXPOSE 6660
ENTRYPOINT ["bash", "-c", "cd /OS; for ((;;)) do nc -l -p 6660 > patch; git pull; git apply patch; make os.iso; nc -l -p 6660 -q 0 < kernel.elf; nc -l -p 6660 -q 0 < os.iso; make clean; git reset --hard HEAD; done"]
