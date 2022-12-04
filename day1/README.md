You need zig@0.10.0 installed to run these

```bash
$ cat input.txt | zig run ./part1.zig
$ cat input.txt | zig run ./part2.zig
```

POST MORTEM:
It's fine. What's annoying is that I was using Windows and couldn't for the life of me not understand why I couldn't parse each line into an integer. Eventually I remembered the Windows line endings, so I just switched to using WSL instead.

- Using the types at @comptime is a pretty nifty feature.
- `try` syntax when looping is quite nice.
- I don't like memory management. I started setting up these data structures with allocators, but then just switched to using the stack exclusively.
- The VSCode plugin kept breaking on me and I wasn't getting much feedback.

All in all, I'll probably go back to making somethign in zig again.
