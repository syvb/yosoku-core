# Yosoku

*Yosoku* is an append-only financial ledger, intended for use in financial applications such as prediction markets. Currently his is more of an idea than an actual implementation; the code in this repo is very minimal.

## How it works

All Yosoku data is stored in an append-only ledger, which is a list of transaction. `Ledger` is a trait, so there can be multiple different implementations of a ledger (e.g. `MemoryLedger`, `PostgresLedger`).

### What's in a ledger

A ledger is a list of transactions. A transaction has a list of postings and some metadata (monotonic creation time, creator, and arbitrary metadata). A posting increases/decreases a specified account by a specified amount. The postings of a transaction always sum to zero.

#### Example
```
Ledger
  Transaction
    # Create user 1 with 100¤ initial balance 
    Creator: SYSTEM_ACCOUNT
    Status: Finalized at Jan 1 2024 00:00
    Data: AccountCreation(Account(1), User)
    Postings:
      SYSTEM_ACCOUNT: -100¤
      Account(1): +100¤
  Transaction
    # Create market
    Creator: Account(1)
    Status: Finalized at Jan 1 2024 00:03
    Data: AccountCreation(Account(2), Contract("Will Twilight Sparkle win my Best Pony award?", ...))
    Postings:
      # initial subsidy
      Account(1): -20¤
      Account(2): +20¤
  Transaction
    # Bet 5¤ on YES
    Creator: Account(1)
    Status: Finalized at Jan 1 2024 00:06
    Data: Bet(Account(2), Outcome::YES)
    Postings:
      Account(1): -6¤
      # Pay 5¤ to contract
      Account(2): +5¤
      # Pay fee to system
      SYSTEM_ACCOUNT: +1¤
  # ... 115¤ of other bets ...
  Transaction
    # Resolve market
    Creator: Account(1)
    Status: Finalized at Jan 2 2024 00:06
    Data: Resolve(Account(2), Outcome::YES)
    Postings:
      # Reduce market balance to zero
      Account(2): -120¤
      # Send payouts using market balance
      Account(1): +10¤
      Account(3):  +30¤
      Account(4):  +80¤
```

### Ledger implementations

`MemoryLedger` is the simplest ledger - it just[^ml-time] stores a list of transactions. Most queries on a `MemoryLedger` is slow - for example, `MemoryLedger::account_balance` has to look at every transaction in the ledger to find out which ones affect the provided account. `PostgresLedger` (coming soon!) is much more efficent, by using various indexes and caches to make commonly used queries faster. However, it always produces the same results as a memory ledger.

Since all ledger implementations implement the same `Ledger` interface, user code doesn't need to care what ledger implementation is being used. You can use a `MemoryLedger` for local testing, a `SqliteLedger` in dev, and a `PostgresLedger` in production - they are indistinguishable from the perspective of user code.

[^ml-time]: It also stores a time number (incremented on every transaction), so that transaction times are deterministic.
