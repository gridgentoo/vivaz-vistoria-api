use redis::{RedisResult, Commands};
use crate::configs::reader_cfg::RedisConfig;
use redis::cluster::{ ClusterClient};


pub struct RepoClient{
    pub db: ClusterClient,

}

impl RepoClient{
    pub fn new(settings: &RedisConfig) -> RedisResult<ClusterClient> {
        let nodes = &settings.redis_uris;
        ClusterClient::open(nodes.clone())

    }
}


pub struct RepoList{
    pub value: Vec<String>,
    pub key: String,
    pub ttl: usize,
}

impl RepoList {
    pub fn set(data: RepoList, repo_client: RepoClient) -> RedisResult<()>{
        let mut conn = repo_client.db.get_connection().unwrap();
        let exist: i32 = conn.exists(data.key.clone()).unwrap();

        let mut result: RedisResult<i32>;
        if exist == 1{
            result=conn.rpush_exists(data.key.clone(), data.value.clone());

        }else{
         result = conn.rpush(data.key.clone(),data.value.clone());
        }
        println!("{:?}", result);

        if data.ttl > 0{
            redis::cmd("EXPIRE")
                .arg(data.key)
                .arg(data.ttl)
                .query(&mut conn)?;
        }

        Ok(())
    }
    pub fn get(key: String, repo_client: RepoClient)->RedisResult<RepoList>{
        let mut conn = repo_client.db.get_connection().unwrap();
        let info: Vec<String>= redis::cmd("LRANGE")
            .arg(&key)
            .arg(0)
            .arg(-1)
            .query(&mut conn)?;
        Ok(RepoList{
            key,
            value: info,
            ttl: 0
        })
    }
    pub fn rem(data: RepoList, repo_client: RepoClient)-> RedisResult<()>{
        let mut conn = repo_client.db.get_connection().unwrap();
        redis::cmd("LREM")
            .arg(data.key.clone())
            .arg(1)
            .arg(data.value[0].clone())
            .query(&mut conn)?;
        Ok(())
    }
}

