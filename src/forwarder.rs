use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;
use std::sync::{Arc, RwLock};

use actix_web::http::{header, Uri};
use actix_web::{web, HttpResponse};
use color_eyre::{eyre::eyre, eyre::WrapErr, Result};
use itertools::Itertools;
use log::{error, info};

#[derive(Debug, Clone)]
pub struct ForwardingTable(Arc<RwLock<HashMap<String, String>>>);

fn url_pairs(line: &str) -> Result<(String, String)> {
    let (from, to) = line
        .split_once(" ")
        .ok_or_else(|| eyre!("Could not split line: {line}"))?;
    let _ = Uri::try_from(from).wrap_err_with(|| format!("Could not parse into Uri: {from}"))?;
    let _ = Uri::try_from(to).wrap_err_with(|| format!("Could not parse into Uri: {to}"))?;
    Ok((from.to_owned(), to.to_owned()))
}

/// spawns background function reloading config as needed
impl ForwardingTable {
    const PATH: &'static str = "forwarded.txt";

    fn setup_reload_on_change(self) -> Result<()> {
        use hotwatch::{Event, Hotwatch};
        let mut hotwatch = Hotwatch::new().wrap_err("Could not enable live reloading")?;

        hotwatch
            .watch(Self::PATH, move |event: Event| {
                if let Event::Write(path) = event {
                    info!("reloading url forwarding from: {path:?}");
                    if let Err(e) = self.reload() {
                        error!("ran into err reloading: {e:?}");
                    }
                }
            })
            .wrap_err("failed to watch file")
    }

    fn reload(&self) -> Result<()> {
        let content =
            fs::read_to_string(Self::PATH).wrap_err("could not open file: {Self::PATH}")?;

        let (new_map, errs): (HashMap<_, _>, Vec<_>) = content
            .lines()
            .skip_while(|s| s.starts_with("#"))
            .skip_while(|s| s.is_empty())
            .map(url_pairs)
            .partition_result();
        for err in errs {
            error!("{err:?}");
        }

        let mut writer = self.0.write().unwrap();
        *writer = new_map;
        Ok(())
    }

    pub fn init() -> Self {
        let table = HashMap::new();
        let table = Self(Arc::new(RwLock::new(table)));
        if let Err(e) = table.reload() {
            error!("{e:?}")
        }
        if let Err(e) = table.clone().setup_reload_on_change() {
            error!("{e:?}");
        }
        table
    }
}

pub async fn route(data: web::Data<ForwardingTable>, path: web::Path<String>) -> HttpResponse {
    let table = data.0.read().unwrap();
    match table.get(&path.into_inner()) {
        Some(forward_to) => HttpResponse::TemporaryRedirect()
            .append_header((header::LOCATION, forward_to.as_str()))
            .finish(),
        None => HttpResponse::NotFound().finish(),
    }
}
