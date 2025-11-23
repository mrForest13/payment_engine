# Payment Engine
Simple payment engine. Supporting:
- Deposit
- Withdrawal
- Dispute
- Resolve
- Chargeback

## Assumptions
- Deposit amount cannot be less than 0
- Withdrawal amount cannot be less than 0
- Deposit amount cannot be negative
- Withdrawal amount cannot be negative
- Deposit precision has to be 4 or less
- Withdrawal precision has to be 4 or less
- Withdrawals cannot be disputed (no funds to held?)
- Trades id are globally unique (No double spending checks)
- Accounts can be process concurrently (no transfers from one account to another)

## How it works?

### Logs can be enabled using 

Uncomment tracing_subscriber::fmt::init();

### Csv reading

The system starts by loading a CSV file using [`CsvReader`](./src/input/csv.rs). The records are loaded into a raw model called [`TransactionRow`](./src/input/row.rs), where an initial validation is also performed to ensure the data is correct — for example, that a `Withdrawal` or `Deposit` has an `amount`. Based on the [`TransactionType`](./src/input/row.rs), the raw model is then transformed into the business model [`Transaction`](./src/model/trade.rs).

### Workers

Next, the transactions are sent one by one to the [`PaymentEngine`](./src/core/engine.rs), which maintains a pool of workers. The function used to select the appropriate [`Worker`](./src/core/worker.rs) is `client_id % pool_size`, which ensures that transactions for the same `account` are processed in order, while allowing different accounts to be processed in `parallel`.

### Core Logic

The [`AccountWallet`](./src/core/wallet.rs) is responsible for applying individual transactions to a given `account`. It also stores the list of `deposits` and `disputes` associated with that account.

### Reporting

When the [`CsvReader`](./src/input/csv.rs) finishes processing the CSV file, the worker engine waits for all workers to return the current state of their accounts. Once all states are collected, the engine generates the final [`Report`](./src/model/report.rs).