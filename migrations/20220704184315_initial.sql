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