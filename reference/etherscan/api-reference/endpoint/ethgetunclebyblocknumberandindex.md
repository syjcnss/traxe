> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# eth_getUncleByBlockNumberAndIndex

> Get uncle block details by block number and index.

export const chain = '1';

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="1">
  Chain ID to query, eg `1` for Ethereum, `8453` for Base from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="proxy">
  Set to `proxy` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="eth_getUncleByBlockNumberAndIndex">
  Set to `eth_getUncleByBlockNumberAndIndex` for this endpoint.
</ParamField>

<ParamField query="tag" type="string" default="0xC63276">
  Block number in hex format.
</ParamField>

<ParamField query="index" type="string" default="0x0">
  Position of the uncle in the block, in hex.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=proxy&action=eth_getUncleByBlockNumberAndIndex&tag=0xC63276&index=0x0&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "jsonrpc": "2.0",
    "id": 1,
    "result": {
      "baseFeePerGas": "0x65a42b13c",
      "difficulty": "0x1b1457a8247bbb",
      "extraData": "0x486976656f6e2063612d68656176792059476f6e",
      "gasLimit": "0x1ca359a",
      "gasUsed": "0xb48fe1",
      "hash": "0x1da88e3581315d009f1cb600bf06f509cd27a68cb3d6437bda8698d04089f14a",
      "logsBloom": "0xf1a360ca505cdda510d810c1c81a03b51a8a508ed601811084833072945290235c8721e012182e40d57df552cf00f1f01bc498018da19e008681832b43762a30c26e11709948a9b96883a42ad02568e3fcc3000004ee12813e4296498261619992c40e22e60bd95107c5bd8462fcca570a0095d52a4c24720b00f13a2c3d62aca81e852017470c109643b15041fd69742406083d67654fc841a18b405ab380e06a8c14c0138b6602ea8f48b2cd90ac88c3478212011136802900264718a085047810221225080dfb2c214010091a6f233883bb0084fa1c197330a10bb0006686e678b80e50e4328000041c218d1458880181281765d28d51066058f3f80a7822",
      "miner": "0x1ad91ee08f21be3de0ba2ba6918e714da6b45836",
      "mixHash": "0xa8e1dbbf073614c7ed05f44b9e92fbdb3e1d52575ed8167fa57f934210bbb0a2",
      "nonce": "0x28cc3e5b7bee9866",
      "number": "0xc63274",
      "parentHash": "0x496dae3e722efdd9ee1eb69499bdc7ed0dca54e13cd1157a42811c442f01941f",
      "receiptsRoot": "0x9c9a7a99b4af7607691a7f2a50d474290385c0a6f39c391131ea0c67307213f4",
      "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
      "size": "0x224",
      "stateRoot": "0xde9a11f0ee321390c1a7843cab7b9ffd3779d438bc8f77de4361dfe2807d7dee",
      "timestamp": "0x6110bd1a",
      "transactionsRoot": "0xa04a79e531db3ec373cb63e9ebfbc9c95525de6347958918a273675d4f221575",
      "uncles": []
    }
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).