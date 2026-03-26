> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Check Transaction Receipt Status

> Retrieves the execution status of a specific transaction using its transaction hash.

export const chain = '1';

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="1">
  Chain ID to query, eg `1` for Ethereum, `8453` for Base from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="transaction">
  Set to `transaction` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="gettxreceiptstatus">
  Set to `gettxreceiptstatus` for this endpoint.
</ParamField>

<ParamField query="txhash" type="string" default="0x513c1ba0bebf66436b5fed86ab668452b7805593c05073eb2d51d3a52f480a76">
  Transaction hash to check the receipt status.
</ParamField>

<RequestExample>
  ```bash  theme={null}
  curl "https://api.etherscan.io/v2/api?chainid=1&module=transaction&action=gettxreceiptstatus&txhash=0x513c1ba0bebf66436b5fed86ab668452b7805593c05073eb2d51d3a52f480a76&apikey=YourApiKeyToken"
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": {
      "status": "1"
    }
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).