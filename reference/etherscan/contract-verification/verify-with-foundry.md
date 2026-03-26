> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Verify with Foundry

<Info>
  Ensuring you're using the latest version of Foundry, via `foundryup`
</Info>

[**Foundry**](https://book.getfoundry.sh/) is a tool that helps take the heat off smart contract development steps, including compiling, deploying and finally submitting your contract for verification.

This assumes you already have a [**Foundry project**](https://getfoundry.sh/guides/project-setup/creating-a-new-project)**,** setup, and you're about to deploy and verify next.

## Deploy and Verify

```bash  theme={null}
forge create --broadcast --rpc-url https://rpc.sepolia.ethpandaops.io --private-key YourPrivateKey src/ContractFile.sol:ContractName --verify --verifier etherscan --etherscan-api-key YourApiKeyToken
```

## Verify an Existing Contract

```bash  theme={null}
forge verify-contract --watch --chain sepolia 0x324eca20b358b18e48f2611f7452560ce3b3c1bb src/ContractFile.sol:ContractName --verifier etherscan --etherscan-api-key YourApiKeyToken
```

## Custom Chains

Most chains with an Etherscan explorer are supported using the `--chain` flag.

For (very) new chains that aren't natively supported by Foundry, you can use a custom `--verifier-url`

Append the Etherscan V2 base path and the new chainId, in this example for Stable Testnet(2201).

```bash  theme={null}
forge verify-contract 0x324eca20b358b18e48f2611f7452560ce3b3c1bb src/ContractFile.sol:ContractName --verifier etherscan --verifier-url "https://api.etherscan.io/v2/api?chainid=2201" --etherscan-api-key YourEtherscanApiKey --watch
```

> *This open source integration was shipped by* [*@iainnash*](https://github.com/iainnash) *and the Foundry team*


Built with [Mintlify](https://mintlify.com).