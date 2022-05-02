use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;
use std::sync::{Arc, RwLock};

use actix_web::http::{header, Uri};
use actix_web::{web, HttpResponse};
use color_eyre::{eyre::eyre, eyre::WrapErr, Result};
use itertools::Itertools;
use log::error;

type Table = Arc<RwLock<HashMap<String, String>>>;
#[derive(Debug, Clone)]
pub struct ForwardingTable {
    table: Table,
}

fn url_pairs(line: &str) -> Result<(String, String)> {
    let (from, to) = line
        .split_once(" ")
        .ok_or_else(|| eyre!("Could not split line: {line}"))?;
    let _ = Uri::try_from(from).wrap_err_with(|| format!("Could not parse into Uri: {from}"))?;
    let _ = Uri::try_from(to).wrap_err_with(|| format!("Could not parse into Uri: {to}"))?;
    Ok((from.to_owned(), to.to_owned()))
}

fn reload(table: &Table) -> Result<()> {
    const PATH: &'static str = "forwarded.txt";
    let content = fs::read_to_string(PATH).wrap_err("could not open file: {PATH}")?;

    let (new_map, errs): (HashMap<_, _>, Vec<_>) = content
        .lines()
        .skip_while(|s| s.starts_with("#"))
        .skip_while(|s| s.is_empty())
        .map(url_pairs)
        .partition_result();
    for err in errs {
        error!("{err:?}");
    }

    let mut writer = table.write().unwrap();
    *writer = new_map;
    Ok(())
}


/// spawns background function reloading config as needed
impl ForwardingTable {
    pub fn init() -> Self {
        let table = Arc::new(RwLock::new(HashMap::new()));
        if let Err(e) = reload(&table) {
            error!("{e:?}");
        };
        Self { table}
    }
}

pub async fn route(data: web::Data<ForwardingTable>, path: web::Path<String>) -> HttpResponse {
    let table = data.table.read().unwrap();
    match table.get(&path.into_inner()) {
        Some(forward_to) => HttpResponse::TemporaryRedirect()
            .append_header((header::LOCATION, forward_to.as_str()))
            .finish(),
        None => HttpResponse::NotFound().finish(),
    }
}
