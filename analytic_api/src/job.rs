use crate::pool::PoolState;
use crate::pool::StarrocksConnection;
use r2d2_mysql::mysql::params::Params;
use r2d2_mysql::mysql::prelude::*;
use r2d2_mysql::mysql::Error::MySqlError;
use std::env;
use std::sync::Arc;
use tracing::error;

pub struct Job {
    state: PoolState,
}

impl Job {
    pub fn config(user: &String, pass: &Option<String>, host: &String, port: u64) -> Self {
        Job {
            state: StarrocksConnection::new(
                user.to_owned(),
                pass.to_owned(),
                host.to_owned(),
                port,
                String::new(),
            )
            .unwrap()
            .connect()
            .unwrap(),
        }
    }
    fn create_external_catalog(&self) -> Result<(), Box<dyn std::error::Error>> {
        let pg_user = env::var("POSTGRES_USER").unwrap();
        let pg_pass = env::var("POSTGRES_PASSWORD").unwrap();
        let pg_host = env::var("POSTGRES_HOST").unwrap();
        let pg_port = env::var("POSTGRES_PORT").unwrap();
        let pg_db = env::var("POSTGRES_DB").unwrap();

        let stmt = format!(
            r#"
CREATE EXTERNAL CATALOG stream_catalog
PROPERTIES
(
    "type"="jdbc", 
    "user"="{pg_user}",
    "password"="{pg_pass}",
    "jdbc_uri"="jdbc:postgresql://{pg_host}:{pg_port}/{pg_db}",
    "driver_url"="https://repo1.maven.org/maven2/org/postgresql/postgresql/42.3.3/postgresql-42.3.3.jar",
    "driver_class"="org.postgresql.Driver");"#
        );
        {
            let mut pool = self.state.get().unwrap();
            if let Err(e) = pool.query_drop(stmt) {
                let exception = match e {
                    MySqlError(e) => e.message,
                    _ => String::new(),
                };
                error!("{exception}");
            };
        }
        Ok(())
    }

    pub fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        &self.create_external_catalog().unwrap();
        let stmt = "
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
group by target, cc, total, hist;";

        {
            let mut pool = self.state.get().unwrap();
            let stmt_splits = stmt.split(";").into_iter();
            for stmt in stmt_splits {
                let query = format!("{stmt};");
                if let Err(e) = pool.query_drop(query) {
                    let exception = match e {
                        MySqlError(e) => e.message,
                        _ => String::new(),
                    };
                    error!("{exception}");
                }
            }

            // pool.query_drop(stmt).unwrap();
        }
        Ok(())
    }
}
