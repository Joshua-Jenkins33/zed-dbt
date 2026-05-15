WITH source AS (
    SELECT
        order_id,
        customer_id,
        order_date,
        amount
    FROM {{ source('raw', 'orders') }}
),

renamed AS (
    SELECT
        order_id,
        customer_id,
        CAST(order_date AS DATE) AS ordered_at,
        amount AS order_amount
    FROM source
)

SELECT
    order_id,
    customer_id,
    ordered_at,
    order_amount
FROM renamed
