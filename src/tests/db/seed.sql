INSERT INTO
    quotes ("show", "character", "text")
SELECT
    *
FROM
    json_to_recordset($1 :: json) AS ("show" text, "character" text, "text" text);