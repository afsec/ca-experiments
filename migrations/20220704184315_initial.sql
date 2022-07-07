-- Add migration script here

CREATE TABLE "authors" (
	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	"name"	TEXT NOT NULL
);

CREATE TABLE "publishers" (
	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	"name"	TEXT NOT NULL
);

CREATE TABLE "books" (
	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	"title"	TEXT NOT NULL,
	"price"	INTEGER NOT NULL,
	"quantity"	INTEGER NOT NULL,
	"author"	INTEGER NOT NULL,
	"publisher"	INTEGER NOT NULL,
	FOREIGN KEY("publisher") REFERENCES "publishers"("id"),
	FOREIGN KEY("author") REFERENCES "authors"("id")
);

-- CREATE TABLE "users" (
-- 	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
-- 	"name"	INTEGER NOT NULL,
-- );

CREATE TABLE "orders" (
	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	"name" TEXT NOT NULL
	-- "user" INTEGER NOT NULL,
	-- FOREIGN KEY("user") REFERENCES "users"("id")
);

CREATE TABLE "carts" (
		"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
		"order" INTEGER NOT NULL,
		"book"  INTEGER NOT NULL,
		"price"	INTEGER NOT NULL,
		FOREIGN KEY("order") REFERENCES "orders"("id"),
		FOREIGN KEY("book") REFERENCES "books"("id")
);

