# Payment Engine
Simple payment engine. Supporting:
- Deposit
- Withdrawal
- Dispute
- Resolve
- Chargeback

## How it works?

### Csv reading

The system starts by loading a CSV file using [`CsvReader`](./src/input/csv.rs). The records are loaded into a raw model called [`TransactionRow`](./src/input/row.rs), where an initial validation is also performed to ensure the data is correct — for example, that a `Withdrawal` or `Deposit` has an `amount`. Based on the [`TransactionType`](./src/model/trade.rs), the raw model is then transformed into the business model [`Transaction`](./src/model/trade.rs).