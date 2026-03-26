> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Verify with Hardhat

<Info>
  This guide covers Hardhat 3. There is an official <a href="https://hardhat.org/docs/hardhat3/migration"><strong>migration guide</strong></a> from the Hardhat team.
</Info>

[**Hardhat**](https://hardhat.org/) is a smart contracts development tool, perfect if you're familiar with Javascript/Typescript.

This assumes you already have a [**Hardhat 3 project**](https://hardhat.org/docs/getting-started)**,** setup, and you're about to deploy and verify next.

## Install

Via npm.

```bash  theme={null}
npm add --save-dev @nomicfoundation/hardhat-verify
```

## Config

In `hardhat.config.ts`, import the plugin at the top and add it to the list of `plugins`.

Then, specify a `verify` config with your Etherscan API key. This key works for most [**supported chains**](/supported-chains), otherwise you need to define a CustomChain below.

```ts  theme={null}
import { defineConfig } from "hardhat/config";
import hardhatVerify from "@nomicfoundation/hardhat-verify";

export default defineConfig({
  plugins: [
    hardhatVerify,
    // ...other plugins...
  ],
  solidity: "0.8.28",
  networks: {
    sepolia: {
      url: configVariable("SEPOLIA_RPC_URL"),
      accounts: [configVariable("SEPOLIA_PRIVATE_KEY")],
    },
  },
  verify: {
    etherscan: {
      apiKey: "YourEtherscanApiKey",
    },
  },
});
```

## Deploy and Verify (using Hardhat Ignition)

```bash  theme={null}
npx hardhat ignition deploy ignition/modules/Counter.ts --network sepolia --verify
```

## Verify an Existing Contract

```bash  theme={null}
npx hardhat verify --network sepolia 0xdCBdBAA8404554502B433106e62728B659aCfE3b
```

## Custom Chains

For new chains that have an Etherscan explorer but isn't supported with Hardhat defaults, you need to add both the `networks` and `chainDescriptor` in `hardhat.config.ts`.

```ts  theme={null}
export default defineConfig({
  // ...
  networks: {
    monadTestnet: {
      type: "http",
      chainType: "l1",
      url: configVariable("MONAD_TESTNET_RPC_URL"),
      accounts: [configVariable("MONAD_TESTNET_PRIVATE_KEY")],
    }
  },
  chainDescriptors: {
    10143: {
      name: "monadTestnet",
      blockExplorers: {
        etherscan: {
          name: "Monad Testnet Explorer",
          url: "https://testnet.monadscan.com",
          apiUrl: "https://api.etherscan.io/v2/api",
        },
      },
    }
  },
});
```

Use the same deploy and verify command as above, but with the updated `--network`. In this example for `monadTestnet`

```bash  theme={null}
npx hardhat ignition deploy ignition/modules/Counter.ts --network monadTestnet --verify
```


Built with [Mintlify](https://mintlify.com).