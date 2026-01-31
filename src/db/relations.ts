import { defineRelations } from "drizzle-orm";

import * as schema from "./tables";

export const relations = defineRelations(schema, () => ({
  quotes: {},
  quoteFilters: {},
}));
