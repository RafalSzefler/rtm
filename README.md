Rafal's Transaction Manager (rtm)
=================================

![Build](https://github.com/RafalSzefler/rtm/actions/workflows/tests.yaml/badge.svg)

Project for aggregating and calculating transactions.

Input is a valid csv file with the same format as `transactions.csv` file.
Note that the file is expected to be a valid csv, with each row having exactly
4 fields (even if they are empty): rtm **will silently ignore** any invalid row.

Usage
=====

```
cargo run -- transactions.csv
```

The application will output aggregated data to stdout.

Libs
====

* [`rtm_core`](https://rafalszefler.github.io/rtm/rtm_core)

