You need [Unison](https://www.unison-lang.org/)

```bash
$ ucm
$ load part1.u
$ run part1
$ load part2.u
$ run part2
```

POST MORTEM:
The development experience is quite good, being able to test expressions by just writing `> ...` like `> test` which I used to watch my tests. It's similar to the functionality that's currently in the Haskell language server where you can add `>>>` to a comment; however this felt much smoother. Especially since it's a first class feature and can be type checked. There wasn't any auto formatting in the language server I used, but the syntax is very simple.

For this I had to create my own `u128` type, which was a bit of a hassle. Wish that, or a bit array of a specific length existed that I could use. Oh, well. Thumbs up overall, unison.

I tried using some effects but became bloated, since the scope was so small so I removed it in the end.
