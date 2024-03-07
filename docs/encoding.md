# Hash Encoding

All of the Hashers in this crate returns an `ImageHash` struct that can be encoded into a hexadecimal string.
This document dives a bit deeper into this encoding and explains how it works.

_This encoding algorithm is explicitly choosen to be compatible with the Python `imagehash` package while also supporting encoding of a non-square matrix._

