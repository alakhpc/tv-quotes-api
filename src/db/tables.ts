import { integer, pgTable, text } from "drizzle-orm/pg-core";

export const quotes = pgTable("quotes", {
  id: integer("id").primaryKey().generatedAlwaysAsIdentity(),
  show: text("show").notNull(),
  character: text("character").notNull(),
  text: text("text").notNull(),
});

// Pre-computed ID arrays for O(1) random selection
export const quoteFilters = pgTable("quote_filters", {
  key: text("key").primaryKey(), // "all", "short", "show:breaking bad", "show:breaking bad:short"
  quoteIds: integer("quote_ids").array().notNull(),
  count: integer("count").notNull(),
});
