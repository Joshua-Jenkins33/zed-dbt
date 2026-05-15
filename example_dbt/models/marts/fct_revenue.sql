{% set min_amount = var('min_amount', 0) %}

WITH orders AS (
    SELECT
        order_id,
        customer_id,
        ordered_at,
        order_amount
    FROM {{ ref('stg_orders') }}
    WHERE order_amount >= {{ min_amount }}
),

daily_revenue AS (
    SELECT
        ordered_at,
        COUNT(*) AS order_count,
        SUM(order_amount) AS revenue
    FROM orders
    GROUP BY ordered_at
)

SELECT
    ordered_at,
    order_count,
    revenue
FROM daily_revenue
ORDER BY ordered_at
