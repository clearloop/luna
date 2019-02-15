# Details
Thoughts of Implementation.

## Concepts
### Transation
Confusing about if transaction is necessary, cryptocurrency, or equal value good, maybe we can use the `trust` part of transactions and create something more genius???

### Database
`Database` challenge is quite serious, at the present stage,  implementing a simple whole file I/O is the way `luna` chose.

+ Storage Structure
  
  `Barrel` contains `head` and `body`, `transactions` storage in body, `head` eats the `hash` and `nonce` which is set after `proof-of-work`.

+ Merkle Tree
  
  Merkle Tree is necessary, but, not for now.

### Consensus
Used to consider about the `cuckoo` consensus, proof-of-work or others, in a word, is not the root question, consensus question is the question about trust, and in common `blockchain`, is about the longest `chain`, world consensus is horrible, obversely, blockchain is valuable, and, `private-chain` might be more simple.

### Crypto
Just like `database`, `crypto` is also a quite serious challenge, for now, wraps `ed25519-dalek` directly, rebuild it after `consensus`.

### Virtual Machine
Virtual Machine is the __hardcore__ part in luna, as the same of other thoughts, neither `wasm` nor `risc-v` now, exploring the essence of using.

<br>

## Idle
Refer fo `layer` model, during `Proof of Concept`.

### Layer Structure
+ Primitive
  
  Depends on rust `std` lib, and awesome crates from rust developers, contains the basic code of `Luna`

+ Thruster
  
  Layer depends on `primitive`, embarrassed now, `miner` and `guardian` traits for `cowboy` at `primitive`, `revolver` as the IO utils trait.

+ Spaceboy
  
  A mod that gathered `thruster`, from now on, got these two methods:
    + shoot - send transactions
    + pack - mine barrel.
    
what about replace `mine` to `drink`??? 

### Transfer
+ [x] Load Account
+ [x] Make Transaction
+ [x] Push Transaction to Pool
+ [ ] Network Sync
+ [ ] Assets Change
+ [ ] Call Back?

### Mine
+ [x] Load Account
+ [x] Sync Transaction Pool
+ [x] Pack Transaction
+ [x] Proof of Work
+ [x] Stretch chain
+ [ ] Reward
+ [ ] Broadcast
+ [ ] Consensus

### Contract
+ [ ] Offline Programming
+ [ ] Push to Machine
+ [ ] Assets Modify
+ [ ] Machine computing
+ [ ] State Convert

<br>

## To Do
### Network
Basic Network Interface, implement tcp server/client at mod `primitive`.

### Transaction
+ [x] Sign
+ [x] Verify
+ [ ] `usize` problem

### Miner
+ [ ] Coinbase
+ [ ] Miner Special Reward

<br>

## In Progress
### UTXO
A layer aims to scan barrel-chain,  checkout the value of each account, located at mod `spaceboy`, like a tracker, named `telescope` in luna, yep, cowboy's telescope.

<br>

## Reflecting Thoughts
### Times
"Blockchain" time, isn't it? people talking about "Blockchain", and the developers developing operating system without prior consultation, secretly, something is ending, and, some changing is happening.

### Developers
A dream you dream alone is only a dream, a dream you dream together is reality, sort thoughts ourselves, keep moving, too many questions we can't solve by oneself, keep talking, help each other, and we can make this dream come true.

### Users
Any thoughts which becomes common starts from a human, create what we need, find out if others need, if we satisfied ourselves, help others.

### Economy
Recycling Economy, about open source project, there is a lot of words, public the code and the project, no need profits, trade with others on the fork inside the ecology, is fair.

### Society
The potential users, or, the affected people indirectly, how do you feel? what do you concerned about? and, what's the relationship between us?
