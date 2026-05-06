# Tag-Based File Manager (Rust)

## Overview

This project is a high-performance, tag-based file manager written in Rust.
Files are organized using tags instead of folders, allowing flexible categorization where a file can belong to multiple groups.

The system works as a drop-in layer on top of existing file systems:

* No modification of actual files
* No embedded metadata in files
* Fully compatible with existing folder structures

All metadata is stored in an embedded database.

## Goal

Provide a fast, responsive file browsing experience where users can:

* Assign multiple tags to files
* Incrementally filter files by selecting tags
* Instantly see matching files after each interaction

The system is optimized for:

* Fast read performance
* Incremental UI updates
* Efficient storage of tag relationships

## Core Idea

The system uses an inverted index:

```
tag → set of file IDs
```

Example:

```
tag:work      → {1,4,7,9}
tag:projectA  → {1,7,12}
```

Query:

```
work AND projectA
```

Execution:

```
{1,4,7,9} ∩ {1,7,12} = {1,7}
```

## Architecture

### Storage Layer (LMDB)

* file_id → file_path
* tag_name → tag_id
* tag_id → RoaringBitmap<file_id>

### Index Layer

* Each tag maps to a Roaring Bitmap of file IDs
* Stored in LMDB as a compact binary value

### Query Engine

* Load bitmap for selected tag
* Intersect with current result set
* Map resulting file IDs to file paths

## Incremental Filtering

User interaction model:

```
click tag1 → result A
click tag2 → result A ∩ B
click tag3 → result A ∩ B ∩ C
```

Implementation:

```
current_result &= new_tag_bitmap
```

* Only one bitmap is loaded per interaction
* Result set continuously shrinks
* No full recomputation required

## Roaring Bitmaps

Used to store sets of file IDs efficiently.

Features:

* Compressed representation of integer sets
* Fast intersection using bitwise operations
* Low memory usage for sparse data
* Microsecond-level query performance

## Data Flow

1. Scan filesystem → assign file_ids
2. Store file paths in LMDB
3. Store tags as bitmaps of file IDs
4. On user interaction:
    * Load bitmap for selected tag
    * Intersect with current result
    * Map file IDs to paths
5. Display results

## Dependencies

### Core

* Rust
* heed (LMDB binding)
* roaring (Roaring Bitmaps)

### Supporting

* walkdir or ignore (filesystem scanning)
* serde / bincode (serialization)
