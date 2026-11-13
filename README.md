# Norka Terminal

This terminal is designed for professional users who know how to program in Rust. I am developing it for my own personal use and needs.

What makes it special? It scales well to suit your specific tasks, and in the future, it will be capable of handling multiple tasks simultaneously. Currently, for instance, it can look up weather information based on your request—pulling data from sources that either do or do not require authentication.

Can you trust it? That is entirely up to you. Personally, I would trust it with relatively simple tasks—the kind I’m just too lazy to do myself. However, you could potentially teach it to do almost anything—even using Qiskit as an engine for quantum-secure "coin flips" to decide your fate (after all, how else would you determine whether or not to order pizza tonight?).

How do you run it? That depends entirely on how you intend to use it. I work on NixOS, so my setup process differs slightly from yours—though not drastically. The steps are outlined below.

```
  $ nix develop
  $ cargo build
  $ cargo run
```

if you're not using NixOS, probably this will work for you:

```
  $ cargo build
  $ cargo run
```
