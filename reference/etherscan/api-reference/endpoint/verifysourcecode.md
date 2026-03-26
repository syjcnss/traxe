> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Verify Solidity Source Code

> Submit a Solidity source code for verification.

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

<ParamField query="action" type="string" default="verifysourcecode">
  Set to `verifysourcecode` for this endpoint.
</ParamField>

<ParamField query="contractaddress" type="string" default="0xBB9bc244D798123fDe783fCc1C72d3Bb8C189413">
  The address where the contract is deployed.
</ParamField>

<ParamField query="sourceCode" type="string">
  The Solidity source code to verify.
</ParamField>

<ParamField query="codeformat" type="string" default="solidity-standard-json-input">
  Use `solidity-single-file` for a single file or `solidity-standard-json-input` for JSON input.
</ParamField>

<ParamField query="contractname" type="string" default="contracts/Verified.sol:Verified">
  The contract name, including path if applicable. If `codeformat=solidity-standard-json-input`, then enter contractname as `erc20.sol:erc20`.
</ParamField>

<ParamField query="compilerversion" type="string" default="v0.8.24+commit.e11b9ed9">
  Compiler version used for compilation.
</ParamField>

<ParamField query="optimizationUsed" type="string" default="0">
  Use `1` if optimization was used or `0` if disabled, specify runs below.
</ParamField>

<ParamField query="runs" type="string" default="200">
  Number of optimization runs.
</ParamField>

<ParamField query="constructorArguments" type="string" default="00000000000000000000000074271f2282ed7ee35c166122a60c9830354be42a">
  Optional constructor arguments used in contract deployment.
</ParamField>

<ParamField query="evmVersion" type="string" default="default">
  Use compiler `default` or specify an EVM version such as `byzantium`, `shanghai`.
</ParamField>

<ParamField query="licenseType" type="string" default="1">
  The [open source license](https://etherscan.io/contract-license-types) to associate with the verified source code, e.g `3` for MIT.
</ParamField>

<ResponseExample>
  ```json Response theme={null}
  {
      "status": "1",
      "message": "OK",
      "result": "a7lpxkm9kpcpicx7daftmjifrfhiuhf5vqqnawhkfhzfrcpnxj"
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).