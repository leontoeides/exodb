Atlatl is a performance-conscious wrapper for redb with ergonomic access to typed records, multiple serializers, index support, and range queries. Built for precision, clarity, and speed.

# Error Correction

Atlatl includes optional per-value error correction using Reed-Solomon codes. This allows you to safeguard individual records (such as documents, media assets, or user data) against silent corruption and disk-level bit rot.

## Features

* Granular protection: ECC is applied at the value level, not at the file, table, or disk level.
* Seamless recovery: Errors are automatically corrected during reads with zero downtime. Your application never knows corruption occurred.
* Configurable: Enable it globally, per table, or disable it entirely. Tune it to fit your performance and integrity needs.
* Transparent: Works on any filesystem or storage backend. No dependency on ZFS, RAID, or special hardware.

## What It Protects Against

* Bit rot: Silent corruption from aging storage media.
* Partial disk failures: Readable sectors with corrupted data.
* Hardware glitches: Transient errors in disk controllers, cables, or RAM.
* Cosmic rays: Random bit flips in memory or storage.

## Limitations

* Metadata vulnerability: Only the value payload is protected by ECC. If [redb](https://www.redb.org/)'s structural metadata, indexes, or the ECC metadata itself gets corrupted, Atlatl cannot restore those parts.
* Capacity limits: Reed-Solomon can only correct a limited number of errors per record. Massive corruption beyond the correction threshold will still result in data loss.
* Performance cost: ECC adds computational overhead and storage overhead from parity data.
* Less efficient on small values: For small records, ECC introduces measurable space and compute overhead with less protection benefit. It performs best with larger blobs (PDFs, images, documents).
* Not a silver bullet: ECC complements but does not replace traditional data protection strategies like offsite backups, snapshots, RAID, or ZFS.

## When To Use

Enable ECC when:
* Data integrity is more critical than raw performance or disk efficiency
* You're storing valuable assets (documents, media, configurations)
* You want protection against silent corruption that other layers might miss

Bottom line: ECC won't make your data immortal, but it can save the day in many real-world corruption scenarios that would otherwise require restoring from backups.

# Credits

Developed by Dylan Bowker and the Atlatl Team, with strategic insight from Ariadne.