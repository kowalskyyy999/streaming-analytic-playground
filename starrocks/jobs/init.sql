CREATE EXTERNAL CATALOG stream_catalog
PROPERTIES
(
    "type"="jdbc", 
    "user"="kowalskyyy",
    "password"="Password123456",
    "jdbc_uri"="jdbc:postgresql://db:5432/stream_db",
    "driver_url"="https://repo1.maven.org/maven2/org/postgresql/postgresql/42.3.3/postgresql-42.3.3.jar",
    "driver_class"="org.postgresql.Driver"
);

create database stream_db;
use stream_db;

create view v_user_champs
as
select username, target, count_click, sc, finish_time
from (
  select 
    b.username, 
    a.target, 
    a.count_click,  
    date_diff('second', a.finish_time, a.start_time) as sc, 
    a.finish_time + interval 7 hour as finish_time 
  from stream_catalog.public.history_champs a 
  left join stream_catalog.public.users b on a.id = b.id) base 
group by username, target, count_click, sc, finish_time;

create materialized view mv_fastest
distributed by hash(target, username, ranking)
refresh async every (interval 1 minute)
as
select target, username, sc times, rank() over(partition by target order by sc) ranking 
from (
  select username, target, sc 
  from v_user_champs 
  group by username, target, sc) 
base order by target, ranking;

create materialized view mv_fastest_today
distributed by hash(target, username, ranking)
refresh async every (interval 1 minute)
as
select target, username, sc times, rank() over(partition by target order by sc) ranking 
from (
  select username, target, sc 
  from v_user_champs 
  where date(finish_time) = date(now())
  group by username, target, sc) 
base order by target, ranking;

create materialized view mv_shortest
distributed by hash(target, username, ranking)
refresh async every (interval 1 minute)
as
select target, username, cc as clicks, rank() over(partition by target order by cc) ranking 
from (
  select username, target, count_click cc
  from v_user_champs 
  group by username, target, count_click) 
base order by target, ranking;

create materialized view mv_shortest_today
distributed by hash(target, username, ranking)
refresh async every (interval 1 minute)
as
select target, username, cc as clicks, rank() over(partition by target order by cc) ranking 
from (
  select username, target, count_click cc
  from v_user_champs
  where date(finish_time) = date(now())
  group by username, target, count_click) 
base order by target, ranking;

create materialized view mv_hist_fastest
distributed by hash(target, sc)
refresh async every (interval 1 minute)
as
with base as (
      select username, target, sc
      from v_user_champs
      group by username, target, sc),
    hist as (
      select target, sc, 
        count(*) over(partition by target) as total, 
        count(*) over(partition by target, sc) / count(*) over(partition by target) * 100 as hist
      from base)
select target, sc, total, hist
from hist
group by target, sc, total, hist;


create materialized view mv_hist_shortest
distributed by hash(target, cc)
refresh async every (interval 1 minute)
as
with base as (
      select username, target, count_click cc
      from v_user_champs
      group by username, target, count_click),
    hist as (
      select target, cc, 
        count(*) over(partition by target) as total, 
        count(*) over(partition by target, cc) / count(*) over(partition by target) * 100 as hist
      from base)
select target, cc, total, hist 
from hist
group by target, cc, total, hist;
