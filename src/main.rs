mod adapters;
mod configs;
mod domain;
mod service;
use crate::configs::reader_cfg::{RedisConfig, SettingsReader};

use crate::service::list_service::{get_list, map_repo_list, set_list, rem_list, map_repo_list_full, map_payload_to_repo_list};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use serde::{Deserialize, Serialize};
use crate::domain::{responders::List, request::Message};
use crate::adapters::eventstore::producer;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SETTINGS: SettingsReader = SettingsReader::new("Settings.toml", "");
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Parameters {
    #[serde(rename = "type")]
    tip: String,
}

async fn get_horarios_disponiveis(
    web::Path((mes, ano)): web::Path<(String, String)>,
    data: web::Data<&RedisConfig>,
)-> HttpResponse{
    let mut r : List= List{ list: None,};
    let key = String::from("Disponiveis::".to_owned()+ &mes + "::" + &ano);

    let l: Vec<String> = get_list(&data, map_repo_list(key.clone()));
    if l.is_empty() {
        return HttpResponse::NotFound().body("");
    }else{
        r.list = Option::from(l);
    }

    HttpResponse::Ok().body(serde_json::to_string_pretty(&r).unwrap())
}

async fn get_agendar_horario(
    web::Path((mes, ano, init, end)): web::Path<(String, String, String, String)>,
    data: web::Data<&RedisConfig>,
)-> HttpResponse{

    let kd = String::from("Disponiveis::".to_owned()+ &mes + "::" + &ano);
    let ki = String::from("Indisponiveis::".to_owned()+ &mes + "::" + &ano);
    let mut l: Vec<String> = vec![];
    l.push(String::from(init.to_owned() + "::" + &end));

    let _= rem_list(&data,map_repo_list_full(kd,l.clone()));
    let set = set_list(&data, map_repo_list_full(ki,l.clone()));
    producer(String::from(l[0].clone()));
    match set{
        Ok(_)=>{
            return HttpResponse::Ok().body("Success");
        }
        Err(_)=>{
            return HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }


}

async fn inserir_orarios_disponiveis(
    web::Path((mes, ano, init, end)): web::Path<(String, String, String, String)>,
    data: web::Data<&RedisConfig>,
)-> HttpResponse{

    let kd = String::from("Disponiveis::".to_owned()+ &mes + "::" + &ano);
    let mut l: Vec<String> = vec![];
    l.push(String::from(init.to_owned() + "::" + &end));


    let set = set_list(&data, map_repo_list_full(kd,l));

    match set{
        Ok(_)=>{
            return HttpResponse::Ok().body("Success");
        }
        Err(_)=>{
            return HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

async fn set_key(
    data: web::Data<&RedisConfig>,
    param: web::Query<Parameters>,
    path: web::Path<String>,
    info: web::Json<Message>,
) -> HttpResponse {

    let key: String = get_key_from_path(path.to_string());
    match param.tip.as_str() {
        "list" => set_list(&data, map_payload_to_repo_list(&info, key)).unwrap(),
        _ => {}
    };
    HttpResponse::NoContent().body("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_config = &SETTINGS.redis;

    HttpServer::new(move || {
        App::new().data(redis_config)
            .service(web::resource("/api/keys/{path:.*}")
                .route(web::put().to(set_key))
            )
            .route("/api/scheduler/disponibilidade/{mes}/{ano}", web::get().to(get_horarios_disponiveis))
            .route("/api/scheduler/agendamento/{mes}/{ano}/{init}/{end}", web::put().to(get_agendar_horario))
            .route("/api/scheduler/massa/{mes}/{ano}/{init}/{end}", web::put().to(inserir_orarios_disponiveis))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReturnList{
    list: Vec<String>
}

fn get_key_from_path(s: String) -> String {
    let re = Regex::new(r"/").unwrap();
    let result = re.replace_all(s.as_str(), "::");
    result.to_string()
}
