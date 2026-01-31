import { defineConfig } from "drizzle-kit";

export default defineConfig({
  out: "./src/db/migrations",
  schema: "./src/db/tables.ts",
  dialect: "postgresql",
  migrations: { table: "migrations" },
  dbCredentials: {
    url: process.env.DATABASE_URL,
  },
});
