> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Deposit Transactions by Address

> Retrieves all deposit transactions made by a specified address.

export const chain = '1';

<Callout icon="globe" iconType="regular">This endpoint is only available for the Arbitrum and Optimism stack chains.</Callout>

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="10">
  Chain ID to query, eg `10` for Optimism or `42161` for Arbitrum from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="account">
  Set to `account` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="getdeposittxs">
  Set to `getdeposittxs` for this endpoint.
</ParamField>

<ParamField query="address" type="string" default="0x80f3950a4d371c43360f292a4170624abd9eed03">
  Address to check for cross-chain deposits from Ethereum to L2.
</ParamField>

<ParamField query="page" type="integer" default="1">
  Page number for pagination.
</ParamField>

<ParamField query="offset" type="integer" default="10">
  Number of records per page.
</ParamField>

<ParamField query="sort" type="string" default="desc">
  Sort order either `desc` for the latest transactions first or `asc` for the oldest transactions first.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=10&module=account&action=getdeposittxs&address=0x80f3950a4d371c43360f292a4170624abd9eed03&page=1&offset=10&sort=desc&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": [
      {
        "blockNumber": "132992375",
        "timeStamp": "1741583527",
        "blockHash": "0xef2ff158c8b12be842429f4a8cde58bfa6a389c5274b46f8a1dd2ee7f958ca4d",
        "hash": "0x64ccd0cfa9f333578b36227492f3bc7f5f3ec4bfa82cdc46f82884db680d8e5b",
        "nonce": "520502",
        "from": "0x36bde71c97b33cc4729cf772ae268934f7ab70b2",
        "to": "0x4200000000000000000000000000000000000007",
        "value": "598200000000000",
        "gas": "490798",
        "gasPrice": "0",
        "input": "0xd764ad0b000100000000000000000000000000000000000000000000000000000002802a00000000000000000000000099c9fc46f92e8a1c0dec1b1747d010903e884be10000000000000000000000004200000000000000000000000000000000000001000000000000000000000000000000000000000000000000000002200f4a81300000000000000000000000000000000000000000000000000000000000000030d4000000000000000000000000000000000000000000000000000000000000000000c0000000000000000000000000000000000000000000000000000000000000000a41635f5fd0000000000000000000000001231deb6f5749ef6ce6943a275a1d3e7486f4eae00000000000000000000000080f3950a4d371c43360f292a4170624abd9eed03000000000000000000000000000000000000000000000000000002200f4a813000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "cumulativeGasUsed": "169497",
        "gasUsed": "117234",
        "isError": "0",
        "errDescription": "",
        "txreceipt_status": "1",
        "queueIndex": "999999",
        "L1transactionhash": "0x303bd05c47e62e1243a33210e535ebc70a7567e53a9972fbdef52ee5bcda5acb",
        "L1TxOrigin": "0x36bde71c97b33cc4729cf772ae268934f7ab70b2",
        "tokenAddress": "ETH",
        "tokenSentFrom": "",
        "tokenSentTo": "0x80f3950a4d371c43360f292a4170624abd9eed03",
        "tokenValue": "598200000000000"
      },
      {
        "blockNumber": "134878032",
        "timeStamp": "1745354841",
        "blockHash": "0xd1a47b46e3aaed25c594507cf9df59c9ba7a25542fabed7e2b646efe8b6944f9",
        "hash": "0xb7cbf6eb529f60b28c6707a52e17c84e75bbe0a968e668a29e5a7550bfb00e92",
        "nonce": "527247",
        "from": "0x36bde71c97b33cc4729cf772ae268934f7ab70b2",
        "to": "0x4200000000000000000000000000000000000007",
        "value": "1195403000000000",
        "gas": "490798",
        "gasPrice": "0",
        "input": "0xd764ad0b0001000000000000000000000000000000000000000000000000000000029a8300000000000000000000000099c9fc46f92e8a1c0dec1b1747d010903e884be100000000000000000000000042000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000043f36732dae00000000000000000000000000000000000000000000000000000000000000030d4000000000000000000000000000000000000000000000000000000000000000000c0000000000000000000000000000000000000000000000000000000000000000a41635f5fd0000000000000000000000001231deb6f5749ef6ce6943a275a1d3e7486f4eae00000000000000000000000080f3950a4d371c43360f292a4170624abd9eed030000000000000000000000000000000000000000000000000000043f36732dae000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        "cumulativeGasUsed": "147297",
        "gasUsed": "92234",
        "isError": "0",
        "errDescription": "",
        "txreceipt_status": "1",
        "queueIndex": "999999",
        "L1transactionhash": "0xdca5992b97fcb3d2aa4d06f8c47c0786c0007009d15445712931ee0998f94caa",
        "L1TxOrigin": "0x36bde71c97b33cc4729cf772ae268934f7ab70b2",
        "tokenAddress": "ETH",
        "tokenSentFrom": "",
        "tokenSentTo": "0x80f3950a4d371c43360f292a4170624abd9eed03",
        "tokenValue": "1195403000000000"
      }
    ]
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).