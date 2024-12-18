### Input files
`random.dat` is generated with:
```bash
head -c 1073741824 /dev/urandom > random.dat
```
`wikipedia.bz2` is downloaded from `enwiki-20241101-pages-meta-history1.xml-p1p812.bz2` on [Wikipedia dumps](https://dumps.wikimedia.org/enwiki/20241101/).
```bash
wget "https://dumps.wikimedia.org/enwiki/20241101/enwiki-20241101-pages-meta-history1.xml-p1p812.bz2" -O wikipedia.bz2
```

`access.log` is downloaded from [Kaggle](https://www.kaggle.com/datasets/eliasdabbas/web-server-access-logs?resource=download).

- XZ is really slow, even at minimal level
- `.bytes()` can sometimes be really slow if you're not careful
- ZSTD needs no input or output buffering on stream decoders
- We tried measuring the elapsed time from before opening the file, and from the beginning of reading data. However, the overhead of opening the file and initializing decompression was negligible.
- Log files need higher compression levels to compress effectively
- Hard drives tend to be at least 10x slower than macbook SSDs