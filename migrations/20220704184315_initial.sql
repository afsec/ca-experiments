-- Add migration script here

CREATE TABLE "author" (
	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	"name"	TEXT
);

CREATE TABLE "publisher" (
	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	"name"	TEXT
);

CREATE TABLE "books" (
	"id"	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	"title"	TEXT,
	"price"	INTEGER,
	"quantity"	INTEGER,
	"author"	INTEGER,
	"publisher"	INTEGER,
	FOREIGN KEY("publisher") REFERENCES "publisher"("id"),
	FOREIGN KEY("author") REFERENCES "author"("id")
);