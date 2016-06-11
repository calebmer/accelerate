begin;

delete from bar.bar where bar = 'bar';

drop table bar.bar;
drop schema bar;

commit;
