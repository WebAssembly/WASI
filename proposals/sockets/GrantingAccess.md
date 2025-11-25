## Granting access

This section is mostly here to illustrate the granularity of permissions that ought to be possible. It is by no means a recommendation of any kind. It's just spitballing how a CLI-based implementation might grant network access.


```shell

# Allow TCP connections to any IP address resolved from `example.com` on port 80. This also implies `--allow-resolve=example.com`
--allow-outbound=tcp://example.com:80

# Allow listening only on loopback interfaces on port 80
--allow-inbound=tcp://localhost:80





# Allow the lookup of a specific domain name
--allow-resolve=example.com

# Allow the lookup of all subdomains
--allow-resolve=*.example.com

# Allow any lookup
--allow-resolve=*

# Only look up IPv4 addresses
--allow-resolve=example.com#ipv4-only

# Only look up IPv6 addresses
--allow-resolve=example.com#ipv6-only




# Allow TCP connections to 127.0.0.1 on port 80
--allow-outbound=tcp://127.0.0.1:80

# Allow TCP connections to 127.0.0.1 on any port
--allow-outbound=tcp://127.0.0.1:*

# Allow TCP connections to any server on port 80
--allow-outbound=tcp://*:80

# Allow all TCP connections
--allow-outbound=tcp://*:*

# Allow TCP connection with IPv4 only
--allow-outbound=tcp://...#ipv4-only

# Allow TCP connection with IPv6 only
--allow-outbound=tcp://...#ipv6-only

# Allow TCP connections to a specific list of ports
--allow-outbound=tcp://*:80,443

# Allow TCP connections to a range of ports
--allow-outbound=tcp://*:21,35000-35999

# Allow UDP client
--allow-outbound=udp://...




# Allow listening on a specific network interface on port 80
--allow-inbound=tcp://eth0:80

# Allow listening on any network interface on port 80
--allow-inbound=tcp://*:80

# Allow listening on a randomly generated port
--allow-inbound=tcp://*:0

```

### Virtualization / mapping

Just like wasmtime already has the ability to remap directories with `--mapdir`, similar constructs can be conceived for networking. Examples:

_Again: not an official recommendation of any kind._

```shell

# Map a domain name resolvable from within the wasm module to an IP address.
--allow-resolve=my-database.internal->172.17.0.14

# Allow listening to TCP port 80 inside the wasm module, which is mapped to port 8888 on the host.
--allow-inbound=tcp://*:80->8888

# Allow TCP connections to any IP address resolved from `my-database.internal` which is mapped to `172.17.0.14` on
# port 5432 which is mapped to 5433. This also implies `--allow-resolve=my-database.internal->172.17.0.14`
--allow-outbound=tcp://my-database.internal->172.17.0.14:5432->5433

```