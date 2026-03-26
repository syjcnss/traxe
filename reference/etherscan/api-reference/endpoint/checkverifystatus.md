> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Check Source Code Verification Status

> Check the status of a source code verification request.

export const chain = '1';

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" type="string" default="1">
  Chain ID to query, eg `1` for Ethereum, `8453` for Base from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" type="string" default="contract">
  Set to `contract` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="checkverifystatus">
  Set to `checkverifystatus` for this endpoint.
</ParamField>

<ParamField query="guid" type="string" default="x3ryqcqr1zdknhfhkimqmizlcqpxncqc6nrvp3pgrcpfsqedqi">
  The GUID received from the verification request.
</ParamField>

<ResponseExample>
  ```json Response theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": "Pass - Verified"
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).