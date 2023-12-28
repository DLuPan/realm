# xtunnel

A simple, high performance relay server written in rust.

<p><img src="https://raw.githubusercontent.com/zhboner/xtunnel/master/assets/xtunnel.png"/></p>

[![xtunnel](https://github.com/zhboner/xtunnel/workflows/ci/badge.svg)](https://github.com/zhboner/xtunnel/actions)
[![xtunnel](https://github.com/zhboner/xtunnel/workflows/build/badge.svg)](https://github.com/zhboner/xtunnel/actions/workflows/cross_compile.yml)
[![downloads](https://img.shields.io/github/downloads/zhboner/xtunnel/total?color=green)](https://github.com/zhboner/xtunnel/releases)
<!-- [中文说明](https://zhb.me/xtunnel) -->

## Libs

| lib | doc |
| ----- | ----- |
| xtunnel-core | [![crates.io](https://img.shields.io/crates/v/xtunnel_core.svg)](https://crates.io/crates/xtunnel_core) [![Released API docs](https://docs.rs/xtunnel_core/badge.svg)](https://docs.rs/xtunnel_core) |
| xtunnel-io | [![crates.io](https://img.shields.io/crates/v/xtunnel_io.svg)](https://crates.io/crates/xtunnel_io) [![Released API docs](https://docs.rs/xtunnel_io/badge.svg)](https://docs.rs/xtunnel_io) |
| xtunnel-lb | [![crates.io](https://img.shields.io/crates/v/xtunnel_lb.svg)](https://crates.io/crates/xtunnel_lb) [![Released API docs](https://docs.rs/xtunnel_lb/badge.svg)](https://docs.rs/xtunnel_lb) |
| xtunnel-hook | [![crates.io](https://img.shields.io/crates/v/xtunnel_hook.svg)](https://crates.io/crates/xtunnel_hook) [![Released API docs](https://docs.rs/xtunnel_hook/badge.svg)](https://docs.rs/xtunnel_hook)|
| xtunnel-syscall | [![crates.io](https://img.shields.io/crates/v/xtunnel_syscall.svg)](https://crates.io/crates/xtunnel_syscall) [![Released API docs](https://docs.rs/xtunnel_syscall/badge.svg)](https://docs.rs/xtunnel_syscall) |

## Features

- Zero configuration. Setup and run in one command.
- Concurrency. Bidirectional concurrent traffic leads to high performance.
- Low resources cost.

## Container

xtunnel can be run in a container with OCI (like Docker, Podman, Kubernetes, etc), see guides [here](readme.container.md).

## Build

Install rust toolchain with [rustup](https://rustup.rs/):

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone this repository:

```shell
git clone https://github.com/zhboner/xtunnel && cd xtunnel
```

Build:

```shell
cargo build --release
```

You can also pass `target_cpu=native` to allow more possible optimizations:

```shell
RUSTFLAGS='-C target_cpu=native' cargo build --release
```

The `xtunnel` binary will be available in `target/release`.

### Build Options

- ~~udp: enable udp relay~~ builtin.
- ~~tfo: enable tcp-fast-open~~ deprecated.
- ~~trust-dns: enable trust-dns's async dns resolver~~ builtin.
- ~~zero-copy: enable zero-copy on linux~~ builtin.
- brutal-shutdown: see [xtunnel_io/brutal-shutdown](xtunnel_io/README.md#about-brutal-shutdown).
- hook: see [xtunnel_hook](xtunnel_hook/README.md).
- proxy: enable proxy-protocol.
- balance: enable load balance.
- transport: enable ws/tls/wss.
- multi-thread: enable tokio's multi-threaded IO scheduler.
- mi-malloc: custom memory allocator.
- jemalloc: custom memory allocator.
- page-alloc: custom memory allocator.

Default: proxy + balance + transport + brutal-shutdown + multi-thread.

See also: [Cargo.toml](Cargo.toml).

Examples:

```shell
# simple tcp
cargo build --release --no-default-features

# enable other options
cargo build --release --features 'jemalloc'

# fully customized
cargo build --release
    --no-default-features
    --features 'transport, multi-thread, jemalloc'
```

### Cross Compile

Please refer to [https://rust-lang.github.io/rustup/cross-compilation.html](https://rust-lang.github.io/rustup/cross-compilation.html). You may need to install cross-compilers or other SDKs, and specify them when building the project.

Or have a look at [Cross](https://github.com/cross-rs/cross), it makes things easier.

## Usage

```shell
xtunnel 2.5.0 [proxy][balance][brutal][transport][multi-thread]
A high efficiency relay tool

USAGE:
    xtunnel [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       show help
    -v, --version    show version
    -d, --daemon     run as a unix daemon
    -u, --udp        force enable udp forward
    -t, --ntcp       force disable tcp forward
    -f, --tfo        force enable tcp fast open -- deprecated
    -z, --splice     force enable tcp zero copy -- deprecated

OPTIONS:
    -c, --config <path>                 use config file
    -l, --listen <address>              listen address
    -r, --remote <address>              remote address
    -x, --through <address>             send through ip or address
    -i, --interface <device>            bind to interface
    -a, --listen-transport <options>    listen transport
    -b, --remote-transport <options>    remote transport

SYS OPTIONS:
    -n, --nofile <limit>          set nofile limit
    -p, --pipe-page <number>      set pipe capacity
    -j, --pre-conn-hook <path>    set pre-connect hook

LOG OPTIONS:
        --log-level <level>    override log level
        --log-output <path>    override log output

DNS OPTIONS:
        --dns-mode <mode>            override dns mode
        --dns-min-ttl <second>       override dns min ttl
        --dns-max-ttl <second>       override dns max ttl
        --dns-cache-size <number>    override dns cache size
        --dns-protocol <protocol>    override dns protocol
        --dns-servers <servers>      override dns servers

PROXY OPTIONS:
        --send-proxy                       send proxy protocol header
        --send-proxy-version <version>     send proxy protocol version
        --accept-proxy                     accept proxy protocol header
        --accept-proxy-timeout <second>    accept proxy protocol timeout

TIMEOUT OPTIONS:
        --tcp-timeout <second>           override tcp timeout(5s)
        --udp-timeout <second>           override udp timeout(30s)
        --tcp-keepalive <second>         override default tcp keepalive interval(15s)
        --tcp-keepalive-probe <count>    override default tcp keepalive count(3)

SUBCOMMANDS:
    convert    convert your legacy configuration into an advanced one
```

Start from command line arguments:

```shell
xtunnel -l 0.0.0.0:5000 -r 1.1.1.1:443
```

Start with a config file:

```shell
# use toml
xtunnel -c config.toml

# use json
xtunnel -c config.json
```

Start with environment variables:

```shell
xtunnel_CONF='{"endpoints":[{"local":"127.0.0.1:5000","remote":"1.1.1.1:443"}]}' xtunnel

# or
export xtunnel_CONF=`cat config.json | jq -c `
xtunnel
```

Convert a legacy config file:

```shell
xtunnel convert old.json
```

## Configuration

TOML Example

```toml
[log]
level = "warn"
output = "/var/log/xtunnel.log"

[network]
no_tcp = false
use_udp = true

[[endpoints]]
listen = "0.0.0.0:5000"
remote = "1.1.1.1:443"

[[endpoints]]
listen = "0.0.0.0:10000"
remote = "www.google.com:443"

```

<details>
<summary>JSON Example</summary>
<p>

```json
{
  "log": {
    "level": "warn",
    "output": "/var/log/xtunnel.log"
  },
  "network": {
    "no_tcp": false,
    "use_udp": true
  },
  "endpoints": [
    {
      "listen": "0.0.0.0:5000",
      "remote": "1.1.1.1:443"
    },
    {
      "listen": "0.0.0.0:10000",
      "remote": "www.google.com:443"
    }
  ]
}

```

</p>
</details>

[See more examples here](./examples).

## Overview

```shell
├── log
│   ├── level
│   └── output
├── dns
│   ├── mode
│   ├── protocol
│   ├── nameservers
│   ├── min_ttl
│   ├── max_ttl
│   └── cache_size
├── network
│   ├── no_tcp
│   ├── use_udp
│   ├── tcp_timeout
│   ├── udp_timeout
│   ├── tcp_keepalive
│   ├── tcp_keepalive_probe
│   ├── send_proxy
│   ├── send_proxy_version
│   ├── accept_proxy
│   └── accept_proxy_timeout
└── endpoints
    ├── listen
    ├── remote
    ├── extra_remotes
    ├── balance
    ├── through
    ├── interface
    ├── listen_transport
    ├── remote_transport
    └── network->
```

You should provide at least [endpoint.listen](#endpointlisten-string) and [endpoint.remote](#endpointremote-string), the left fields will take their default values.

Option priority: cmd override > endpoint config > global config.

### endpoint

#### endpoint.listen: string

Local address, supported formats:

- ipv4:port
- ipv6:port

#### endpoint.remote: string

Remote address, supported formats:

- ipv4:port
- ipv6:port
- example.com:port

#### endpoint.extra_remotes: string array

Extra remote address, same as endpoint.remote above.

#### endpoint.balance: string

Require `balance` feature.

Load balance strategy and weights of remote peers.

Format:

```bash
$strategy: $weight1, $weight2, ...
```

Where `remote` is used as default backend server, and `extra_remotes` are used as backups.

Available algorithms (provided by [xtunnel_lb](./xtunnel_lb/)):

- iphash

- roundrobin

Example:

```toml
[[endpoints]]
remote = "a:443"
extra_remotes = ["b:443", "c:443"]
balance = "roundrobin: 4, 2, 1"
```

The weight of [a, b, c] is [4, 2, 1] in turn.

#### endpoint.through: string

TCP: Bind a specific `ip` before opening a connection.

UDP: Bind a specific `ip` or `address` before sending packet.

Supported formats:

- ipv4/ipv6 (tcp/udp)
- ipv4/ipv6:port (udp)

#### endpoint.interface: string

Bind to a specific interface.

#### endpoint.listen_transport: string

Require `transport` feature.

See [Kaminari Options](https://github.com/zephyrchien/kaminari#options).

#### endpoint.remote_transport: string

Require `transport` feature.

See [Kaminari Options](https://github.com/zephyrchien/kaminari#options).

#### endpoint.network

The same as [network](#network), override global options.

### log

#### log.level: string

values:

- off
- error
- warn
- info
- debug
- trace

default: off

#### log.output: string

values:

- stdout
- stderr
- path (e.g. `/var/log/xtunnel.log`)

default: stdout

### dns

Require `trust-dns` feature.

#### dns.mode: string

Dns resolve strategy.

values:

- ipv4_only
- ipv6_only
- ipv4_then_ipv6
- ipv6_then_ipv4
- ipv4_and_ipv6

default: ipv4_and_ipv6

#### dns.protocol: string

Dns transport protocol.

values:

- tcp
- udp
- tcp_and_udp

default: tcp_and_udp

#### dns.nameservers: string array

Custom upstream servers.

format: ["server1", "server2" ...]

default:

If on **unix/windows**, read from the default location.(e.g. `/etc/resolv.conf`).

Otherwise, use google's public dns(`8.8.8.8:53`, `8.8.4.4:53` and `2001:4860:4860::8888:53`, `2001:4860:4860::8844:53`).

#### dns.min_ttl: unsigned int

The minimum lifetime of a positive dns cache.

default: 0

#### dns.max_ttl: unsigned int

The maximum lifetime of a positive dns cache.

default: 86400 (1 day)

#### dns.cache_size: unsigned int

The maximum count of dns cache.

default: 32

### network

#### network.no_tcp: bool

Do not start a tcp relay.

default: false

#### network.use_udp: bool

~~Require `udp` feature~~

Start listening on a udp endpoint and forward packets to the remote peer.

It will dynamically allocate local endpoints and establish udp associations. Once timeout, the endpoints will be deallocated and the association will be terminated. See also: [network.udp_timeout](#networkudp_timeout-unsigned-int).

Due to the receiver side not limiting access to the association, the relay works like a full-cone NAT.

default: false

#### ~~network.zero_copy: bool~~ deprecated

~~Require `zero-copy` feature.~~

~~Use `splice` instead of `send/recv` while handing tcp connection. This will save a lot of memory copies and context switches.~~

~~default: false~~

#### ~~network.fast_open: bool~~ deprecated

~~Require `fast-open` feature.~~

~~It is not recommended to enable this option, see [The Sad Story of TCP Fast Open](https://squeeze.isobar.com/2019/04/11/the-sad-story-of-tcp-fast-open/).~~

~~default: false~~

#### network.tcp_timeout: unsigned int

This is **connect** timeout. An attempt to connect to a remote peer fails after waiting for a period of time.

To disable timeout, you need to explicitly set timeout value to 0.

default: 5

#### network.udp_timeout: unsigned int

Terminate udp association after `timeout`.

The timeout value must be properly configured in case of memory leak. Do not use a large `timeout`!

default: 30

#### network.tcp_keepalive: unsigned int

TCP Keepalive interval.

On Linux, this is equivalent to setting both `net.ipv4.tcp_keepalive_time` and `net.ipv4.tcp_keepalive_intvl`.

To use system's tcp keepalive interval, you need to explicitly set this option to 0.

default: 15

#### network.tcp_keepalive_probe: unsigned int

TCP Keepalive retries.

On Linux, this is equivalent to `ipv4.tcp_keepalive_probes`.

default: 3

#### network.send_proxy: bool

Require `proxy` feature.

Send haproxy PROXY header once the connection established. Both `v1` and `v2` are supported, see [send_proxy_version](#networksend_proxy_version-unsigned-int).

You should make sure the remote peer also speaks proxy-protocol.

default: false

#### network.send_proxy_version: unsigned int

Require `proxy` feature.

This option has no effect unless [send_proxy](#networksend_proxy-bool) is enabled.

value:

- 1
- 2

default: 2

#### network.accept_proxy: bool

Require `proxy` feature.

Wait for a PROXY header once the connection established.

If the remote sender does not send a `v1` or `v2` header before other contents, the connection will be closed.

default: false

#### network.accept_timeout: unsigned int

Require `proxy` feature.

Wait for a PROXY header within a period of time, otherwise close the connection.

default: 5.
