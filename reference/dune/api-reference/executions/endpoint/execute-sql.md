> ## Documentation Index
> Fetch the complete documentation index at: https://docs.dune.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Execute SQL

> Execute raw SQL queries directly without needing to create a saved query first

> Run arbitrary SQL queries via API. This endpoint allows you to execute SQL directly without first creating a query object, perfect for dynamic analysis and automated workflows.

<Note>
  When executing arbitrary SQL queries (queries not saved in Dune), the `query_id` in the response will always be `0`. This is a placeholder value since these queries are not stored in the Dune database.
</Note>

## Pricing

This endpoint is **usage-based** and consumes credits based on the computational resources used during execution.

<RequestExample>
  ```bash cURL theme={null}
  curl -X POST "https://api.dune.com/api/v1/sql/execute" \
    -H "Content-Type: application/json" \
    -H "X-Dune-Api-Key: YOUR_API_KEY" \
    -d '{
      "sql": "SELECT * FROM dex.trades WHERE block_time > now() - interval '\''1'\'' day LIMIT 10",
      "performance": "medium"
    }'
  ```

  ```python Python theme={null}
  import requests

  url = "https://api.dune.com/api/v1/sql/execute"
  headers = {
      "X-DUNE-API-KEY": "YOUR_API_KEY",
      "Content-Type": "application/json"
  }
  data = {
      "sql": "SELECT * FROM dex.trades WHERE block_time > now() - interval '1' day LIMIT 10",
      "performance": "medium"
  }

  response = requests.post(url, json=data, headers=headers)
  print(response.json())
  ```

  ```javascript JavaScript theme={null}
  const response = await fetch('https://api.dune.com/api/v1/sql/execute', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'X-DUNE-API-KEY': 'YOUR_API_KEY'
    },
    body: JSON.stringify({
      sql: "SELECT * FROM dex.trades WHERE block_time > now() - interval '1' day LIMIT 10",
      performance: 'medium'
    })
  });

  const data = await response.json();
  console.log(data);
  ```

  ```bash Complete Workflow theme={null}
  # Step 1: Execute SQL and get execution ID
  EXECUTION_ID=$(curl -s -X POST "https://api.dune.com/api/v1/sql/execute" \
    -H "Content-Type: application/json" \
    -H "X-Dune-Api-Key: YOUR_API_KEY" \
    -d '{"sql": "SELECT * FROM dex.trades LIMIT 10", "performance": "medium"}' \
    | jq -r '.execution_id')

  # Step 2: Poll for completion
  while true; do
    STATE=$(curl -s "https://api.dune.com/api/v1/execution/$EXECUTION_ID/status" \
      -H "X-Dune-Api-Key: YOUR_API_KEY" \
      | jq -r '.state')
    
    echo "State: $STATE"
    [[ "$STATE" =~ (QUERY_STATE_COMPLETED|QUERY_STATE_FAILED|QUERY_STATE_CANCELLED) ]] && break
    sleep 2
  done

  # Step 3: Get results
  curl -s "https://api.dune.com/api/v1/execution/$EXECUTION_ID/results" \
    -H "X-Dune-Api-Key: YOUR_API_KEY" | jq
  ```
</RequestExample>

<ResponseExample>
  ```json  theme={null}
  {
    "execution_id": "01HKZJ2683PHF9Q9PHHQ8FW4Q1",
    "state": "QUERY_STATE_PENDING"
  }
  ```
</ResponseExample>


## OpenAPI

````yaml POST /v1/sql/execute
openapi: 3.0.1
info:
  title: DuneAPI
  description: Dune API
  contact: {}
  version: '1.0'
servers:
  - url: https://api.dune.com/api
security: []
paths:
  /v1/sql/execute:
    post:
      summary: Execute raw SQL query
      description: Execute raw SQL query without requiring a stored query ID
      parameters:
        - name: X-Dune-Api-Key
          in: header
          description: API Key for the service
          required: true
          schema:
            type: string
        - name: api_key
          in: query
          description: Alternative to using the X-Dune-Api-Key header
          schema:
            type: string
      requestBody:
        description: ExecuteSQLRequest
        content:
          '*/*':
            schema:
              $ref: '#/components/schemas/models.ExecuteSQLRequest'
        required: true
      responses:
        '200':
          description: OK
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/models.ExecuteQueryResponse'
        '400':
          description: Bad Request
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/models.Error400'
        '401':
          description: Unauthorized
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/models.Error401'
        '402':
          description: Payment Required
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/models.Error402'
        '403':
          description: Forbidden
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/models.Error403'
        '404':
          description: Not Found
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/models.Error404'
        '500':
          description: Internal Server Error
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/models.Error500'
components:
  schemas:
    models.ExecuteSQLRequest:
      required:
        - sql
      type: object
      properties:
        performance:
          type: string
          description: The performance engine tier the execution will be run on
          enum:
            - medium
            - large
        sql:
          type: string
          description: The SQL query to execute
    models.ExecuteQueryResponse:
      type: object
      properties:
        execution_id:
          type: string
          example: 01HKZJ2683PHF9Q9PHHQ8FW4Q1
        state:
          type: string
          example: QUERY_STATE_PENDING
    models.Error400:
      type: object
      properties:
        error:
          type: string
          example: Bad Request
    models.Error401:
      type: object
      properties:
        error:
          type: string
          example: Invalid API Key
    models.Error402:
      type: object
      properties:
        error:
          type: string
          example: >-
            This API request would exceed your configured limits per billing
            cycle.
    models.Error403:
      type: object
      properties:
        error:
          type: string
          example: >-
            Not allowed to execute query. Query is archived, unsaved or not
            enough permissions
    models.Error404:
      type: object
      properties:
        error:
          type: string
          example: Object not found
    models.Error500:
      type: object
      properties:
        error:
          type: string
          example: Internal error

````

Built with [Mintlify](https://mintlify.com).