import { eq, sql } from "drizzle-orm";
import { Hono } from "hono";

import { createDb } from "./db";
import { quoteFilters, quotes } from "./db/tables";

const app = new Hono<{ Bindings: CloudflareBindings }>({ strict: false });

// Build filter key from query params
function getFilterKey(show?: string, short?: boolean): string {
  if (show && short) return `show:${show.toLowerCase()}:short`;
  if (show) return `show:${show.toLowerCase()}`;
  if (short) return "short";
  return "all";
}

// GET / - API info
app.get("/", (c) => {
  return c.json({
    name: "TV Quotes API",
    docs: "https://github.com/jepcd/tv-quotes-api",
  });
});

// GET /quotes/shows - List all shows (must be before /quotes/:number)
app.get("/quotes/shows", async (c) => {
  const db = createDb(c.env);
  const result = await db.selectDistinct({ show: quotes.show }).from(quotes);

  return c.json({ shows: result.map((r) => r.show) });
});

// GET /quotes/stats - Quote count (must be before /quotes/:number)
app.get("/quotes/stats", async (c) => {
  const db = createDb(c.env);
  const result = await db.query.quoteFilters.findFirst({ columns: { count: true }, where: { key: "all" } });
  return c.json({ total: result?.count ?? 0 });
});

// GET /quotes - Single random quote
app.get("/quotes", async (c) => {
  const db = createDb(c.env);
  const show = c.req.query("show");
  const short = c.req.query("short") === "true";
  const filterKey = getFilterKey(show, short);

  // O(1) random quote using pre-computed ID array
  const result = await db.query.quotes.findFirst({
    columns: { show: true, character: true, text: true },
    where: {
      RAW: (quotes, { eq }) =>
        eq(
          quotes.id,
          sql`(
            SELECT ${quoteFilters.quoteIds}[floor(random() * ${quoteFilters.count} + 1)::int]
            FROM ${quoteFilters}
            WHERE ${eq(quoteFilters.key, filterKey)}
          )`,
        ),
    },
  });

  if (!result) {
    return c.text(`No quotes found for show ${show}`, 404);
  }

  return c.json(result);
});

// GET /quotes/:number - Multiple random quotes (max 10)
app.get("/quotes/:number", async (c) => {
  const db = createDb(c.env);
  const limit = Math.min(parseInt(c.req.param("number")) || 1, 10);
  const show = c.req.query("show");
  const short = c.req.query("short") === "true";
  const filterKey = getFilterKey(show, short);

  // O(limit) random quotes using pre-computed ID array
  const result = await db.query.quotes.findMany({
    columns: { show: true, character: true, text: true },
    where: {
      RAW: (quotes, { inArray }) =>
        inArray(
          quotes.id,
          sql`(
            SELECT DISTINCT ${quoteFilters.quoteIds}[floor(random() * ${quoteFilters.count} + 1)::int]
            FROM ${quoteFilters}, generate_series(1, ${limit * 2})
            WHERE ${eq(quoteFilters.key, filterKey)}
            LIMIT ${limit}
          )`,
        ),
    },
  });

  if (result.length === 0) {
    return c.text(`No quotes found for show ${show}`, 404);
  }

  return c.json(result);
});

export default app;
