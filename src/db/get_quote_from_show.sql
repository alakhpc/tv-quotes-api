SELECT
    *
FROM
    quotes
WHERE
    lower(show) = lower($1)
ORDER BY
    random()
LIMIT
    $2