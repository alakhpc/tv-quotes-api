SELECT
    CEILING(
        random() * (
            SELECT
                MAX(id)
            from
                quotes
        )
    ) as id