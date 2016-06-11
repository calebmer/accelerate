begin;

delete from foo.foo where foo = 'foo';

drop table foo.foo;
drop schema foo;

commit;
