> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get API Usage

> Returns your current API credit usage and limit for the interval.

### Query Parameters

<ParamField query="apikey" type="string" default="YourApiKeyToken">
  Your Etherscan API key.
</ParamField>

<ParamField query="module" type="string" default="getapilimit">
  Set this to `getapilimit` for this endpoint.
</ParamField>

<ParamField query="action" type="string" default="getapilimit">
  Set this to `getapilimit` for this endpoint.
</ParamField>

<ResponseExample>
  ```json  theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": {
      "creditsUsed": 22,
      "creditsAvailable": 1499978,
      "creditLimit": 1500000,
      "limitInterval": "daily",
      "intervalExpiryTimespan": "08:42:34"
    }
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).