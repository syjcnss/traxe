> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Get Address Funded By

> Retrieves the address and transaction that first funded a specific EOA address, useful for tracing fund origins. Not available to contract addresses.

export const TryEndpointLink = ({href, label = 'Try this endpoint in your', linkText = 'browser'}) => {
  if (!href) {
    return null;
  }
  const isExternal = (/^https?:/i).test(href);
  return <p className="mt-4 text-base font-medium text-zinc-900 dark:text-zinc-100">
      {label}{' '}
      <a href={href} className="font-semibold text-primary-600 hover:underline" target={isExternal ? '_blank' : undefined} rel={isExternal ? 'noopener noreferrer' : undefined}>
        {linkText}
      </a>
      .
    </p>;
};

export const chainCurrency = 'ETH';

export const chain = '1';

<Note>This is a PRO endpoint, available to the [Standard Plan](/resources/rate-limits) and above</Note>

<Warning>This endpoint is throttled to **2 calls/second** regardless of API Pro tier.</Warning>

### Query Parameters

<ParamField query="apikey" default="YourApiKeyToken" type="string">
  Your Etherscan API key.
</ParamField>

<ParamField query="chainid" default="1" type="string">
  Chain ID to query, eg `1` for Ethereum, `137` for Polygon from our [supported chains](/supported-chains).
</ParamField>

<ParamField query="module" default="account" type="string">
  Set to `account` for this endpoint.
</ParamField>

<ParamField query="action" default="fundedby" type="string">
  Set to `fundedby` for this endpoint.
</ParamField>

<ParamField query="address" default="0x4838B106FCe9647Bdf1E7877BF73cE8B0BAD5f97" type="string">
  The address to query, like `0x4838B106FCe9647Bdf1E7877BF73cE8B0BAD5f97`.
</ParamField>

<ResponseExample>
  ```json Response theme={null}
  {
    "status": "1",
    "message": "OK",
    "result": {
      "block": 53708500,
      "timeStamp": "1708349932",
      "fundingAddress": "0x6969174fd72466430a46e18234d0b530c9fd5f49",
      "fundingTxn": "0xbc0ca4a67eb1555920552246409626cd60df01314dd2bcdb99718b506d9c9946",
      "value": "1000000000000000"
    }
  }
  ```
</ResponseExample>


Built with [Mintlify](https://mintlify.com).