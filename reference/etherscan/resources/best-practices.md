> ## Documentation Index
> Fetch the complete documentation index at: https://docs.etherscan.io/llms.txt
> Use this file to discover all available pages before exploring further.

# Best Practices

### Limit Return Results

<Tip>Use the `page` and `offset` parameter to request only what you need</Tip>

The `offset` parameter determines how many records are returned from a request.

For example, if your app requires only the latest 10 transactions for an address, setting `offset=10` should see significant speed improvements.

### Limit Your Block Range

<Tip>Use the `startblock` and `endblock` parameter if your block range is known</Tip>

If your block range is known, such as transactions within the [last 24 hours](/api-reference/endpoint/getblocknobytime), your request will benefit from specifying `startblock=<block_number>` and `endblock=latest`.

You can also use the `latest` keyword to specify the most recent block.

### Implement Retry Logic with Backoff

On chain activity (transaction TPS spikes) or temporary high request volumes on shared APIs can occasionally cause timeouts.

If this persists, feel free to [reach out](/resources/contact-us) to us.

### Monitor Usage

<Tip>You will receive an email reminder when you approach your API usage limit</Tip>

From the API dashboard, you can monitor your API usage for suspicious activity, based on the IP and endpoint requests.

If needed, you can monitor your API programmatically via the [Usage](/api-reference/endpoint/getapilimit) endpoint.


Built with [Mintlify](https://mintlify.com).