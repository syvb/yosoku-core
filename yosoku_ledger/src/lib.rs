pub mod cpmm;

/// A transaction is a list of postings that balance (sum to zero).
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub postings: Vec<Posting>,
    pub created_by: Account,
    pub status: TransactionStatus,
    pub data: TransactionData,
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
const SYSTEM_ACCOUNT: Account = Account(0);
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AccountData {
    System,

    // holds tokens
    User,
    Contract,

    // token sources
    BonusSource,
}
pub type TokenNumber = i64;
/// Identifies a signed amount of a currency.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TokenAmount(Token, TokenNumber);
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Token {
    SiteCurrency,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TransactionData {
    AccountCreation(Account),
    Transfer,
}

/// A ledger is a queryable log of transactions.
pub trait Ledger {
    // mutations
    fn transact(&mut self, txn: Transaction);
    fn create_account(&mut self, typ: AccountType, creator: Account) -> Account;

    // queries
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
            txns: vec![
                Transaction {
                    postings: vec![],
                    created_by: SYSTEM_ACCOUNT,
                    status: TransactionStatus::Finalised { time: 0 },
                    typ: TransactionType::AccountCreation,
                    memo: String::new(),
                }
            ],
            time: 0,
        }
    }
    /// This ledger uses simulated time for testing, this function gets the simulated current time.
    pub fn time(&self) -> u64 {
        self.time
    }
    fn next_time(&mut self) -> u64 {
        self.time += 1;
        return self.time;
    }
}
impl Ledger for MemoryLedger {
    fn transact(&mut self, mut txn: Transaction) {
        txn.status = TransactionStatus::Finalised { time: self.next_time() };
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
    fn create_account(&mut self, typ: AccountType, creator: Account) -> Account {
        let now = self.next_time();
        self.transact(Transaction {
            postings: Vec::new(),
            created_by: creator,
            status: TransactionStatus::Finalised { time: now },
            typ: TransactionType::AccountCreation,
            memo: String::new(),
        });
    }
}
