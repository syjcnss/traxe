> ## Documentation Index
> Fetch the complete documentation index at: https://docs.dune.com/llms.txt
> Use this file to discover all available pages before exploring further.

# ethereum.transactions

> Every transaction on Ethereum — from, to, value, gas, calldata, and receipt status.

export const TableSample = ({tableName, tableSchema}) => <>
    <div className="hidden dark:block">
      <iframe src={`https://dune.com/embeds/3419983/5785629?table_schema_t6f0df=${tableSchema}&table_name_t6f0df=${tableName}&darkMode=true`} style={{
  width: '100%',
  height: '500px',
  border: 'none',
  marginTop: '10px'
}} />
    </div>
    <div className="dark:hidden">
      <iframe src={`https://dune.com/embeds/3419983/5785629?table_schema_t6f0df=${tableSchema}&table_name_t6f0df=${tableName}`} style={{
  width: '100%',
  height: '500px',
  border: 'none',
  marginTop: '10px'
}} />
    </div>
  </>;

## Table Description

The `ethereum.transactions` table contains information about all transactions on the Ethereum blockchain. Each row represents a single transaction and includes information such as block number, hash, timestamp, sender, recipient, value, gas, gas price, and more.
Transactions are the fundamental unit of interaction with the Ethereum blockchain. Transactions are created by users and are used to send value, deploy smart contracts, and interact with smart contracts.

This is the raw version of this table, for decoded transaction calls, see the [call tables](/data-catalog/evm/ethereum/decoded/call-tables) section.

## Column Descriptions

| Column                     | Type               | Description                                             |
| -------------------------- | ------------------ | ------------------------------------------------------- |
| `block_time`               | `timestamp`        | Timestamp of the block containing this transaction      |
| `block_number`             | `bigint`           | Block number containing this transaction                |
| `value`                    | `decimal`          | Amount of native token transferred (in wei)             |
| `gas_limit`                | `bigint`           | Maximum gas the sender is willing to use                |
| `gas_price`                | `bigint`           | Price per gas unit (in wei)                             |
| `gas_used`                 | `bigint`           | Actual gas consumed by the transaction                  |
| `max_fee_per_gas`          | `bigint`           | Maximum total fee per gas (EIP-1559)                    |
| `max_priority_fee_per_gas` | `bigint`           | Maximum priority fee (tip) per gas (EIP-1559)           |
| `priority_fee_per_gas`     | `bigint`           | Actual priority fee per gas paid                        |
| `nonce`                    | `bigint`           | Transaction count of the sender before this transaction |
| `index`                    | `bigint`           | Position of the transaction within the block            |
| `success`                  | `boolean`          | Whether the transaction executed successfully           |
| `from`                     | `varbinary`        | Address of the transaction sender                       |
| `to`                       | `varbinary`        | Address of the transaction recipient                    |
| `block_hash`               | `varbinary`        | Hash of the block containing this transaction           |
| `data`                     | `varbinary`        | Calldata sent with the transaction                      |
| `hash`                     | `varbinary`        | Unique hash of the transaction                          |
| `type`                     | `varchar`          | Transaction type (legacy, EIP-2930, EIP-1559, EIP-4844) |
| `access_list`              | `array(row)`       | List of addresses and storage keys accessed (EIP-2930)  |
| `block_date`               | `date`             | Date of the block (UTC)                                 |
| `blob_versioned_hashes`    | `array(varbinary)` | Versioned hashes of blobs (EIP-4844)                    |
| `max_fee_per_blob_gas`     | `bigint`           | Maximum fee per blob gas (EIP-4844)                     |
| `authorization_list`       | `array(row)`       | Authorization list for EIP-7702 transactions            |

## Table Sample

<TableSample tableSchema="ethereum" tableName="transactions" />

## Examples

### Show all transactions sent by a specific address

```sql  theme={null}
select * 
from ethereum.transactions where "from" = 0x50A1b5c358F8D34F0d35aA2f10742c46054E247e
```

### Count the number of transactions per block

```sql  theme={null}
SELECT 
    block_number, 
    COUNT(*)
FROM ethereum.transactions
GROUP BY 1
ORDER BY 1 DESC
LIMIT 10
```

### Show the top 10 transactions with the highest gas price

```sql  theme={null}
SELECT 
    hash, 
    gas_price
FROM ethereum.transactions
ORDER BY gas_price DESC
LIMIT 10
```


Built with [Mintlify](https://mintlify.com).