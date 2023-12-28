drop table if exists "people";
create table "people" ("id" integer primary key, "name" text not null, "age" integer not null);
insert into "people" (name, age) values ('Onat', 21);
insert into "people" (name, age) values ('Aysila', 22);
insert into "people" (name, age) values ('Efe', 71);