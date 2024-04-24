pub mod cpmm;

/// A transaction is a list of postings that balance (sum to zero).
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub postings: Vec<Posting>,
    pub created_by: Account,
    pub status: TransactionStatus,
    pub typ: TransactionType,
    pub memo: String,
}
/// One part of a transaction.
#[derive(Debug, Clone, PartialEq)]
pub struct Posting {
    pub account: Account,
    pub amount: TokenAmount,
}
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionStatus {
    Proposed,
    Finalised { time: u64 },
}
/// Identifies an account.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Account(u64);
pub type TokenNumber = i64;
/// Identifies a signed amount of a currency.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TokenAmount(Token, TokenNumber);
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Token {
    SiteCurrency,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TransactionType {
    Transfer,
}

/// A ledger is a queryable log of transactions.
pub trait Ledger {
    fn transact(&mut self, txn: Transaction);
    fn account_balance(&self, account: Account, token: Token) -> TokenAmount;
}

/// A simple in-memory ledger for testing. Not efficent.
#[derive(Debug)]
pub struct MemoryLedger {
    txns: Vec<Transaction>,
    time: u64,
}
impl MemoryLedger {
    pub fn new() -> Self {
        Self {
            txns: Vec::new(),
            time: 0,
        }
    }
    /// This ledger uses simulated time for testing, this function gets the simulated current time.
    pub fn time(&self) -> u64 {
        self.time
    }
}
impl Ledger for MemoryLedger {
    fn transact(&mut self, mut txn: Transaction) {
        self.time += 1;
        txn.status = TransactionStatus::Finalised { time: self.time };
        self.txns.push(txn.clone());
    }
    fn account_balance(&self, account: Account, token: Token) -> TokenAmount {
        // very inefficent!
        TokenAmount(
            token,
            self.txns
                .iter()
                .map(|txn| {
                    txn.postings
                        .iter()
                        .filter(|posting| posting.amount.0 == token && posting.account == account)
                        .map(|posting| posting.amount.1)
                        .sum::<TokenNumber>()
                })
                .sum(),
        )
    }
}
