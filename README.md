# SpaceJam
[![doc](https://img.shields.io/badge/0.1.0-docs-green.svg)](https://docs.rs/spacejam/)
[![Crates.io](https://img.shields.io/crates/v/spacejam.svg)](https://crates.io/crates/spacejam)
[![Crates.io](https://img.shields.io/crates/d/spacejam.svg)](https://crates.io/crates/spacejam)
[![LICENSE](https://img.shields.io/crates/l/spacejam.svg)](https://choosealicense.com/licenses/mit/)

SpaceJam is a micro-service framework, the implementation is lucid, first for `VM`, second for `Micro-service`.

![you][you]

## How SJ works?

SJ is under [POC](/src/poc) now, here is a demo to show how it works.

Download SJ:

```
$ git clone https://github.com/clearloop/spacejam.git
$ cd spacejam && cargo run
SpaceJam launch at 127.0.0.1:7878...
```

Then we can open another window and use `netcat` to interact with SJ

```
$ (echo 'C(twoSum) (define (twoSum x y) (+ x y))' | nc 127.0.0.1 7878) && echo
ok

$ (echo 'Q(twoSum) (2 2)' | nc 127.0.0.1 7878) && echo
4
```

## Flow

1. [Developer]: Write contracts in sonata.
   1. Send contracts to SpaceJam (with ed25519 account)
2. [SpaceJam]: Receive contracts
   1. Check if contract exist
   2. Parse and write contracts into chain
   3. Serve contracts
	  1. update / stop
3. [Client]: Request contracts with params
4. [SpaceJam]: Receive Request
   1. Decode params from sonata
   2. Return Response
5. [Client]: Get response


### Contracts

How to write a contract? SpaceJam use [sonata](https://github.com/sonata) to parse data, btw, you can impl your own parser using spacejam api.

For example, a `twoSum` contract is like:

```lisp
(contract 
  (int twoSum ((int x) (int y)))
    (+ x y))
```

Then SpaceJam will return us:

```lisp
(map
  (name twoSum)
  (params (int x) (int y))
  (return int))
```

How to send a request to SpaceJam via http/https? `Post` https://example.com/spacejam with body:

```lisp
(map 
  (name twoSum)
  (params 2 2))
```

Returns:

```lisp
(map 
  (name twoSum)
  (params 2 2)
  (data 4))
```

## Architecture

1. Server
2. VM
   1. parser
   2. db
3. p2p

## Contributing
Take your protein pills and put your helmet on, launching yourself into space, you will find us in the tin can beyond the horizon under the velvetground.

## License
GPL-3.0

[you]: https://laughingsquid.com/wp-content/uploads/2018/02/starman-tesla-in-space-towards-mars.gif?w=640
