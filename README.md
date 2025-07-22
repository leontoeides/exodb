Atlatl is a performance-conscious wrapper for redb with ergonomic access to typed records, multiple serializers, index support, and range queries. Built for precision, clarity, and speed.

# Looking For Ecosystem Contributions

* Compression dictionary trainer.
* MySQL Workbench or Microsoft SQL Server Studio type administration software using the debug databases.

# Special Thanks

A special thanks to:
* [Christopher Berner](https://github.com/cberner) and team for the [redb](https://crates.io/crates/redb) crate.
* [David Koloski](https://github.com/djkoloski) and team for the [rkyv](https://crates.io/crates/rkyv) crate.

# The Layer Pipeline

## 1. Serialization

Atlatl hosts a smörgåsbord of serialization options.

### Serialization Features

* `serialize-bincode-native` · [Ty Overby](https://github.com/TyOverby), [Zoey Riordan](https://github.com/ZoeyR) and [Victor Koenders](https://github.com/VictorKoenders)'s [bincode](https://crates.io/crates/bincode) crate's native format
* `serialize-bincode-serde` · [Ty Overby](https://github.com/TyOverby), [Zoey Riordan](https://github.com/ZoeyR) and [Victor Koenders](https://github.com/VictorKoenders)'s [bincode](https://crates.io/crates/bincode) crate's [serde](https://serde.rs/) implementation.
* `serialize-bitcode-native` · [Finn Bear](https://github.com/finnbear) and [Cai Bear](https://github.com/caibear)'s [bitcode](https://crates.io/crates/bitcode) crate's native format.
* `serialize-bitcode-serde` · [Finn Bear](https://github.com/finnbear) and [Cai Bear](https://github.com/caibear)'s [bitcode](https://crates.io/crates/bitcode) crate's [serde](https://serde.rs/) implementation.
* `serialize-borsh` · [NEAR](https://github.com/near)'s [borsh](https://crates.io/crates/borsh) crate.
* `serialize-musli-descriptive` · [John-John Tedro](https://github.com/udoprog)'s [musli](https://crates.io/crates/musli) crate's `descriptive` format.
* `serialize-musli-storage` · [John-John Tedro](https://github.com/udoprog)'s [musli](https://crates.io/crates/musli) crate's `storage` format.
* `serialize-musli-wire ` · [John-John Tedro](https://github.com/udoprog)'s [musli](https://crates.io/crates/musli) crate's `wire` format.
* `serialize-musli-zerocopy` · [John-John Tedro](https://github.com/udoprog)'s [musli-zerocopy](https://crates.io/crates/musli-zerocopy) crate.
* `serialize-postcard-serde` · [James Munns](https://github.com/jamesmunns)' [postcard](https://crates.io/crates/postcard) crate's [serde](https://serde.rs/) implementation.
* `serialize-rkyv` · [David Koloski](https://github.com/djkoloski)'s [rkyv](https://crates.io/crates/rkyv) crate.
* `serialize-messagepack` · MessagePack serialization using [Kornel Lesiński](https://github.com/kornelski) and [Evgeny Safronov](https://github.com/3Hren)'s [rmp-serde](https://crates.io/crates/rmp-serde) crate.
* `serialize-zerocopy` · [Jack Wrenn](https://github.com/jswrenn) and [Joshua Liebow-Feeser](https://github.com/joshlf)'s [zerocopy](https://crates.io/crates/zerocopy) crate.

### Zero-copy Deserialization

Zero-copy deserialization “from the disk to the wire” is possible using the `rkyv`, `musli-zerocopy`, and `zerocopy` serializers, as long as the rest of layer pipeline isn't used. This means compression, encryption, and error correction must be disabled.

## 2. Compression

Atlatl has an optional compression system. All data is compressed at a per-value. Compression can be enabled or disabled, or configured, on a per-table basis. Compression & decompression directions can be controlled - this means that data can be compressed on store, and served from the database in compressed form.

### Compression Features

* `compress-brotli` · Brotli compression using [Servo](https://github.com/servo), [Simon Sapin](https://github.com/SimonSapin), and [Daniel Horn](https://github.com/danielrh)'s [brotli](https://crates.io/crates/brotli) crate.
* `compress-bzip2` · Bzip2 compression using [Alex Crichton](https://github.com/alexcrichton), [bjorn3](https://github.com/bjorn3), and [Folkert de Vries](https://github.com/folkertdev)'s [bzip2](https://crates.io/crates/bzip2) crate.
* `compress-deflate` · DEFLATE compression using [Sebastian Thiel](https://github.com/Byron) and [Josh Triplett](https://github.com/joshtriplett)'s [flate2](https://crates.io/crates/flate2) crate's deflate implementation.
* `compress-gzip` · Gzip compression using [Sebastian Thiel](https://github.com/Byron) and [Josh Triplett](https://github.com/joshtriplett)'s [flate2](https://crates.io/crates/flate2) crate's gzip implementation.
* `compress-lz4` · LZ4 compression using [Pascal Seitz](https://github.com/PSeitz)'s [lz4_flex](https://crates.io/crates/lz4_flex) crate.
* `compress-zlib` · Zlib compression using [Sebastian Thiel](https://github.com/Byron) and [Josh Triplett](https://github.com/joshtriplett)'s [flate2](https://crates.io/crates/flate2) crate's zlib implementation.
* `compress-zstd` · Zstandard compression using [Alexandre Bury](https://github.com/gyscos)'s [zstd](https://crates.io/crates/zstd) crate.

### Dictionaries

When the `compress-dictionaries` feature is enabled, the `lz4`, `zlib`, `zstd` compression algorithms will support dictionary-based compression. Dictionaries can significantly improve compression ratios and speed. This feature is particularly useful when data and data structures are similar and repetitive.

### Warnings

Treat your compression dictionaries like encryption keys:

* If a compression dictionary becomes lost or corrupted, all data will be permanently lost.
* The may contain sensitive information or enough information to decrypt your data.

## 3. Encryption

Atlatl hosts a zero-trust capable encryption system. All data is encrypted at a per-value level. This allows for very high-grain access. Keys can be provided on a per-value, per-table, or per-database level.

## Key Rings

Keys are provided using key-rings which can be chained together and used to decrypt data and can be used to define database security policy.

### Cipher Features

* `encrypt-aes-gcm` · AES-GCM encryption using [Tony Arcieri](https://github.com/tarcieri)'s [aes-gcm](https://crates.io/crates/aes-gcm) crate.
* `encrypt-chacha20` · ChaCha20-Poly1305 encryption using [Artyom Pavlov](https://github.com/newpavlov)'s [chacha20poly1305](https://crates.io/crates/chacha20poly1305) crate.

### Key Derivation Function Features

* `kdf-blake3` · BLAKE3 Key Derivation Function using [Jack O'Connor](https://github.com/oconnor663)'s [blake3](https://crates.io/crates/blake3) crate.
* `kdf-sha256` · SHA-256 Key Derivation Function (KDF) using [Brian Smith](https://github.com/briansmith)'s [ring](https://crates.io/crates/ring) crate.

### Warnings

* If your keys become lost or corrupted, all data will be permanently lost.
* If your keys are leaked, or become public knowledge, your data may be compromised.

## 4. Error Correction

Atlatl includes optional per-value error correction using Reed-Solomon codes. This allows you to safeguard individual records (such as documents, media assets, or user data) against silent corruption and disk-level bit rot.

### Features

* `ecc-reed-solomon` · Reed-Solomon encoding using [Darren Li](https://github.com/darrenldl), [Michael Vines](https://github.com/mvines), and [Nazar Mokrynskyi](https://github.com/nazar-pc)'s [reed-solomon-erasure](https://crates.io/crates/reed-solomon-erasure) crate.

### Benefits

* Granular protection: ECC is applied at the value level, not at the file, table, or disk level.
* Seamless recovery: Errors are automatically corrected during reads with zero downtime. Your application never knows corruption occurred.
* Configurable: Enable it globally, per table, or disable it entirely. Tune it to fit your performance and integrity needs.
* Transparent: Works on any filesystem or storage backend. No dependency on ZFS, RAID, or special hardware.

### What It Protects Against

* Bit rot: Silent corruption from aging storage media.
* Partial disk failures: Readable sectors with corrupted data.
* Hardware glitches: Transient errors in disk controllers, cables, or RAM.
* Cosmic rays: Random bit flips in memory or storage.

### Limitations

* Metadata vulnerability: Only the value payload is protected by ECC. If [redb](https://www.redb.org/)'s structural metadata, indexes, or the ECC metadata itself gets corrupted, Atlatl cannot restore those parts.
* Capacity limits: Reed-Solomon can only correct a limited number of errors per record. Massive corruption beyond the correction threshold will still result in data loss.
* Performance cost: ECC adds computational overhead and storage overhead from parity data.
* Less efficient on small values: For small records, ECC introduces measurable space and compute overhead with less protection benefit. It performs best with larger blobs (PDFs, images, documents).
* Not a silver bullet: ECC complements but does not replace traditional data protection strategies like offsite backups, snapshots, RAID, or ZFS.

### When To Use

Enable ECC when:
* Data integrity is more critical than raw performance or disk efficiency.
* You're storing valuable data or assets (documents, media, configurations).
* You want protection against silent corruption.

Bottom line: ECC won't make your data immortal, but it can save the day in many real-world corruption scenarios that would otherwise require restoring from backups.

# Credits

Developed by Dylan Bowker and the Atlatl Team, with strategic insight from Ariadne.