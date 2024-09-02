# Yosoku

*Yosoku* is an append-only financial ledger, intended for use in financial applications such as prediction markets. Currently his is more of an idea than an actual implementation; the code in this repo is very minimal.

## How it works

All Yosoku data is stored in an append-only ledger, which is a list of transaction. `Ledger` is a trait, so there can be multiple different implementations of a ledger (e.g. `MemoryLedger`, `PostgresLedger`).

### Ledger implementations

`MemoryLedger` is the simplest ledger - it just[^ml-time] stores a list of transactions. Most queries on a `MemoryLedger` is slow - for example, `MemoryLedger::account_balance` has to look at every transaction in the ledger to find out which ones affect the provided account. `PostgresLedger` (coming soon!) is much more efficent, by using various indexes and caches to make commonly used queries faster. However, it always produces the same results as a memory ledger.

[^ml-time]: It also stores a time number (incremented on every transaction), so that transaction times are deterministic.
