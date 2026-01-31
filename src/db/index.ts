import { neon, neonConfig } from "@neondatabase/serverless";
import { drizzle } from "drizzle-orm/neon-http";

import { relations } from "./relations";

neonConfig.fetchEndpoint = (host) => `https://${host}/sql`;

export function createDb(bindings: CloudflareBindings) {
  return drizzle({ client: neon(bindings.DATABASE_URL), relations });
}

export type Database = ReturnType<typeof createDb>;
