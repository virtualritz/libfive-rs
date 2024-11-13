# `libfive` <img src="libfive/libfive-logo.png" alt="libfive logo" width="15%" padding-bottom="5%" align="right" vertical-align="top">

A Rust wrapper for [*libfive*](https://libfive.com/).

> **⚠** ***This is not maintained any more!***
> 
> *You probably want to look at [`fidget`](https://github.com/mkeeter/fidget) instead if you're looking for SDF/implict surfaces in Rust*.

For more information on the high level wrapper see [the README in the
`libfive`](https://github.com/virtualritz/libfive-rs/tree/master/libfive)
folder.

The repositoy comes with minimal dependencies. *libfive* is tracked as a
*Git* submodule under `libfive-sys/libfive`.

Either clone with `--recursive` or, if you already cloned and forgot, simply do
a

```shell
git submodule update --init
```

to pull them in.
