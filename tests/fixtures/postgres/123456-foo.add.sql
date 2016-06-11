begin;

create schema foo;
create table foo.foo (foo text);

insert into foo.foo (foo) values ('foo');

commit;
