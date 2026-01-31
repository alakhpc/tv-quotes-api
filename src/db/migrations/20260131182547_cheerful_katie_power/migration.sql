CREATE TABLE "quote_filters" (
	"key" text PRIMARY KEY,
	"quote_ids" integer[] NOT NULL,
	"count" integer NOT NULL
);
--> statement-breakpoint
CREATE TABLE "quotes" (
	"id" integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY (sequence name "quotes_id_seq" INCREMENT BY 1 MINVALUE 1 MAXVALUE 2147483647 START WITH 1 CACHE 1),
	"show" text NOT NULL,
	"character" text NOT NULL,
	"text" text NOT NULL
);
