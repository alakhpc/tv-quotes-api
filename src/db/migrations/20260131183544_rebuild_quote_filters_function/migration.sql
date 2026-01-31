-- Function to rebuild quote_filters table from existing quotes
-- Call with: SELECT rebuild_quote_filters();

CREATE OR REPLACE FUNCTION rebuild_quote_filters() RETURNS void AS $$
BEGIN
  DELETE FROM quote_filters;
  
  -- All quotes
  INSERT INTO quote_filters (key, quote_ids, count)
  SELECT 'all', array_agg(id ORDER BY id), count(*) FROM quotes;
  
  -- Short quotes (no newlines)
  INSERT INTO quote_filters (key, quote_ids, count)
  SELECT 'short', array_agg(id ORDER BY id), count(*)
  FROM quotes WHERE text NOT LIKE E'%\n%';
  
  -- Per-show (all quotes)
  INSERT INTO quote_filters (key, quote_ids, count)
  SELECT 'show:' || lower(show), array_agg(id ORDER BY id), count(*)
  FROM quotes GROUP BY lower(show);
  
  -- Per-show (short quotes only)
  INSERT INTO quote_filters (key, quote_ids, count)
  SELECT 'show:' || lower(show) || ':short', array_agg(id ORDER BY id), count(*)
  FROM quotes WHERE text NOT LIKE E'%\n%'
  GROUP BY lower(show) HAVING count(*) > 0;
END;
$$ LANGUAGE plpgsql;
