# allup

![rustc](https://img.shields.io/badge/rustc-1.76+-blue?logo=rust)

A CLI to check that all (configured) services are currently up 

## How does it work

Write a tomle file listing all services you care about.

Example: 

```toml
[[endpoints]]
name = "Google"
url = "https://google.com"

[[endpoints]]
name = "Github"
url = "https://github.com"

[[endpoints]]
name = "Rust docs"
url = "https://docs.rs"
```

`allup $FILE` will probe each url (concurrently) and print the status.
It does not fail early if one URL fail (but it does return an error if there was any failure).

The output may look something like this:

```
Google:    UNREACHABLE
Github:    OK (175 ms)
Rust docs: OK (580 ms)
```

> **Note** It is also possible to get the output as JSON.

## Installation

`cargo install allup`

## Maintenance status

I made this project for my personal use. I'll be happy if it can be useful to anyone else, but there are a few things to keep in mind:
* I only write documentation for my "future self", which may not be enough for people who are not in my head.
* The project may never be "feature complete" as I only add new features when I need them.

That being said, I also welcome suggestions and contributions.

## Unlicense

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <http://unlicense.org/>
