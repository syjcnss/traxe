> ## Documentation Index
> Fetch the complete documentation index at: https://docs.dune.com/llms.txt
> Use this file to discover all available pages before exploring further.

# ethereum.traces_decoded

> ABI-decoded function calls on Ethereum — parsed function names, inputs, and outputs from verified contracts.

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

The `traces_decoded` table contains decoded traces from Ethereum transactions. The table matches `MethodID`, which consists of the first 4 bytes of any eth call to the trace signature and the corresponding Method name. The table does not contain the full trace data and is not suitable for analysis of smart contract level data. For that, use the specific event tables as described in [Call-tables](/data-catalog/evm/ethereum/decoded/call-tables).

The `ethereum.traces_decoded` table is great for gaining a high level understanding of the types of events that are being emitted by a smart contract or smart contracts of a certain protocol.

## Column Descriptions

| Column          | Type            | Description                                    |
| --------------- | --------------- | ---------------------------------------------- |
| `block_date`    | `date`          | Date of the block (UTC)                        |
| `block_time`    | `timestamp`     | Timestamp of the block containing this trace   |
| `block_number`  | `bigint`        | Block number containing this trace             |
| `namespace`     | `varchar`       | Project namespace (e.g., uniswap\_v3)          |
| `contract_name` | `varchar`       | Name of the contract that was called           |
| `to`            | `varbinary`     | Address of the contract that was called        |
| `trace_address` | `array(bigint)` | Path of this trace within the call tree        |
| `tx_hash`       | `varbinary`     | Hash of the parent transaction                 |
| `tx_from`       | `varbinary`     | Address that initiated the transaction         |
| `tx_to`         | `varbinary`     | Address the transaction was sent to            |
| `tx_index`      | `integer`       | Position of the transaction within the block   |
| `success`       | `boolean`       | Whether this trace executed successfully       |
| `signature`     | `varbinary`     | Function signature (first 4 bytes of calldata) |
| `function_name` | `varchar`       | Decoded name of the function called            |
| `created_at`    | `timestamp`     | When this record was created in Dune           |

## Table Sample

<TableSample tableSchema="ethereum" tableName="traces_decoded" />


Built with [Mintlify](https://mintlify.com).