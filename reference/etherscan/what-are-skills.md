> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# ELI5 to Skills

[Skills](https://skills.sh/) are instructions to teach AI models how to do a specific task, step by step.

As a dev/product, you would outline a plan to build a [portfolio tracker](https://blockscan.com/address/0x71c7656ec7ab88b098defb751b7401b5f6d8976f) roughly around these steps

1. Fetch native token balances (like ETH for Ethereum)
2. Fetch raw ERC20 token balances and USD price
3. Format the token values by multipling the `raw` values by `decimal` places
4. Sort tokens by the largest USD value
5. Sum all USD values to get a net worth

Having built many of these features ourselves for Etherscan, we've created specific steps and common gotchas in our Skills instructions.

No more excuses for skill issue!

## Connect

<Tabs>
  <Tab title="General">
    You can [copy the .MD page](https://docs.etherscan.io/skill.md) page to any AI of your choice, including ChatGPT.
  </Tab>

  <Tab title="npm">
    Installs a Skill for most [supported AI models](https://skills.sh/), including Claude Code, Codex.

    <Steps>
      <Step title="Install">
        In your terminal, run the command to install Etherscan skills from our docs.

        ```
        npx skills add https://docs.etherscan.io
        ```
      </Step>

      <Step title="Use the $ flag">
        Type `$` which suggests your available Skills, select Etherscan.
      </Step>

      <Step title="Vibe Code">
        Suggest the feature you'd like to work on, chances are we've included steps to guide the AI to build it.

        A good example to start with is

        ```
        $Etherscan build a minimal ETH balance tracker for my address 0x71c7656ec7ab88b098defb751b7401b5f6d8976f
        ```
      </Step>
    </Steps>
  </Tab>
</Tabs>


Built with [Mintlify](https://mintlify.com).