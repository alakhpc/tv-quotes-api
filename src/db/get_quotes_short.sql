SELECT
    *
FROM
    quotes
WHERE
    "text" NOT LIKE E'%\n%'
ORDER BY
    random()
LIMIT
    $1
