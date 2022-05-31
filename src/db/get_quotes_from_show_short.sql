SELECT
    *
FROM
    quotes
WHERE
    lower(SHOW) = lower($1)
    AND "text" NOT LIKE E'%\n%'
ORDER BY
    random()
LIMIT
    $2
