> ## Documentation Index
> Fetch the complete documentation index at: https://docs.dune.com/llms.txt
> Use this file to discover all available pages before exploring further.

# ethereum.traces

> Internal transaction traces on Ethereum — cross-contract calls, delegate calls, and value transfers with full call stack context.

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

The `ethereum.traces` table contains the traces of transactions executed on the Ethereum Virtual Machine (EVM). It includes traces of both top-level and internal transactions. Traces are the result of transaction execution and are used to debug and understand how transactions are processed. They are also known as `internal transactions`.

This is the raw version of this table, for decoded transaction calls, see the [call tables](/data-catalog/evm/ethereum/decoded/call-tables) section.

## Column Descriptions

| Column           | Type            | Description                                         |
| ---------------- | --------------- | --------------------------------------------------- |
| `block_time`     | `timestamp`     | Timestamp of the block containing this trace        |
| `block_number`   | `bigint`        | Block number containing this trace                  |
| `value`          | `uint256`       | Amount of native token transferred (in wei)         |
| `gas`            | `bigint`        | Gas provided for this trace                         |
| `gas_used`       | `bigint`        | Gas consumed by this trace                          |
| `block_hash`     | `varbinary`     | Hash of the block containing this trace             |
| `success`        | `boolean`       | Whether this trace executed successfully            |
| `tx_index`       | `integer`       | Position of the transaction within the block        |
| `sub_traces`     | `bigint`        | Number of child traces spawned by this trace        |
| `error`          | `varchar`       | Error message if the trace failed                   |
| `tx_success`     | `boolean`       | Whether the parent transaction succeeded            |
| `tx_hash`        | `varbinary`     | Hash of the parent transaction                      |
| `from`           | `varbinary`     | Address that initiated this trace                   |
| `to`             | `varbinary`     | Address the trace was sent to                       |
| `trace_address`  | `array(bigint)` | Path of this trace within the call tree             |
| `type`           | `varchar`       | Type of trace (call, create, suicide, reward)       |
| `address`        | `varbinary`     | Address of the contract created (for create traces) |
| `code`           | `varbinary`     | Bytecode of the contract created                    |
| `call_type`      | `varchar`       | Type of call (call, delegatecall, staticcall)       |
| `input`          | `varbinary`     | Input data for the trace                            |
| `output`         | `varbinary`     | Output data from the trace                          |
| `refund_address` | `varbinary`     | Address receiving refund (for selfdestruct)         |
| `block_date`     | `date`          | Date of the block (UTC)                             |

## Table Sample

<TableSample tableSchema="ethereum" tableName="traces" />

## Example

### Querying traces for a specific transaction hash

```sql  theme={null}
SELECT *
FROM ethereum.traces
WHERE tx_hash = 0xb30d6d67cf7d148c2257bf598c5f5cdf5912a3d05c7d3b000398d675d2fa912c
```


Built with [Mintlify](https://mintlify.com).