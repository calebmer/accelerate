begin;

create schema bar;
create table bar.bar (bar text);

insert into bar.bar (bar) values ('bar');

commit;
