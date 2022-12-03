# Dockerust: A Fast, Safe and Resource-friendly Container Runtime written in Rust

## Build

Do not run on your personal computer!

`cargo build --release`

## Get a image

```
docker run busybox
docker ps -a
docker export -o busybox.tar container_name
```

## Run

`target/release/dockerust run -t busybox.tar /bin/ls`

## Get your IP

`ip a`

## Benchmark

```
for run in {1..10}; do time target/release/dockerust run -t busybox.tar /bin/ls > /dev/null; sleep 2; done
for run in {1..10}; do time docker run busybox /bin/ls > /dev/null; sleep 2; done
valgrind --tool=massif --stacks=yes docker run busybox /bin/ls
valgrind --tool=massif --stacks=yes target/release/dockerust run -t busybox.tar /bin/ls

docker run -p 80:80 nginx

target/release/dockerust run -t nginx.tar /bin/sh
# mknod -m 0666 /dev/null c 1 3
# nginx
```
