/*******************************************************
 * Copyright (C) 2019,2020 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
//use packybara::packrat::{Client, NoTls};
use structopt::StructOpt;
mod cmd;
use cmd::args::*;
use env_logger;
use env_logger::Env;
use main_error::MainError;
use std::env;
use tokio;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), MainError> {
    let opt = Pb::from_args();
    if let Pb {
        loglevel: Some(ref level),
        ..
    } = opt
    {
        env::set_var("RUST_LOG", level);
    }
    env_logger::from_env(Env::default().default_filter_or("warn")).init();
    // let mut client = Client::connect(
    //     "host=127.0.0.1 user=postgres dbname=packrat password=example port=5432",
    //     NoTls,
    // )?;

    let (mut client, connection) = tokio_postgres::connect(
        "host=127.0.0.1 user=postgres  dbname=packrat password=example port=5432",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let Pb { crud, .. } = opt;
    match crud {
        PbCrud::Find { cmd } => match cmd {
            PbFind::VersionPin { .. } => {
                cmd::versionpin::find(client, cmd).await?;
            }
            PbFind::VersionPins { .. } => {
                cmd::versionpins::find(client, cmd).await?;
            }
            PbFind::Roles { .. } => {
                cmd::all_roles::find(client, cmd).await?;
            }
            PbFind::Platforms { .. } => {
                cmd::all_platforms::find(client, cmd).await?;
            }
            PbFind::Sites { .. } => {
                cmd::all_sites::find(client, cmd).await?;
            }
            PbFind::Levels { .. } => {
                cmd::all_levels::find(client, cmd).await?;
            }
            PbFind::Pins { .. } => {
                cmd::pins::find(client, cmd).await?;
            }
            PbFind::VersionPinWiths { .. } => {
                cmd::versionpin_withs::find(client, cmd).await?;
            }
            PbFind::Withs { .. } => {
                cmd::withs::find(client, cmd).await?;
            }
            PbFind::Packages { .. } => {
                cmd::all_packages::find(client, cmd).await?;
            }
            PbFind::Distributions { .. } => {
                cmd::all_distributions::find(client, cmd).await?;
            }
            PbFind::PkgCoords { .. } => {
                cmd::pkgcoords::find(client, cmd).await?;
            }
            PbFind::Revisions { .. } => {
                cmd::all_revisions::find(client, cmd).await?;
            }
            PbFind::Changes { .. } => {
                cmd::all_changes::find(client, cmd).await?;
            }
        },
        PbCrud::Add { cmd } => match cmd {
            PbAdd::Packages { .. } => {
                let tx = client.transaction().await?;
                cmd::all_packages::add(tx, cmd).await?;
            }
            PbAdd::Levels { .. } => {
                let tx = client.transaction().await?;
                cmd::all_levels::add(tx, cmd).await?;
            }
            PbAdd::Roles { .. } => {
                let tx = client.transaction().await?;
                cmd::all_roles::add(tx, cmd).await?;
            }
            PbAdd::Platforms { .. } => {
                let tx = client.transaction().await?;
                cmd::all_platforms::add(tx, cmd).await?;
            }
            PbAdd::Withs { .. } => {
                let tx = client.transaction().await?;
                cmd::withs::add(tx, cmd).await?;
            }
            PbAdd::VersionPins { .. } => {
                let tx = client.transaction().await?;
                cmd::versionpins::add(tx, cmd).await?;
            }
        },
        PbCrud::Set { cmd } => match cmd {
            PbSet::VersionPins { .. } => {
                let tx = client.transaction().await?;
                cmd::versionpins::set(tx, cmd).await?;
            }
        },
        PbCrud::Export { cmd } => match cmd {
            PbExport::PackagesXml { .. } => {
                cmd::export::export(client, cmd).await?;
            }
        },
        _ => println!("Not implemented"),
    }

    Ok(())
}
