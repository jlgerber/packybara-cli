/*******************************************************
 * Copyright (C) 2019,2020 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use super::args::{PbAdd, PbFind};
use packybara::db::traits::*;
use packybara::packrat::{Client, PackratDb, Transaction};
use packybara::traits::TransactionHandler;
use packybara::OrderRoleBy;
use prettytable::{cell, format, row, table};
use std::ops::Deref;
use std::str::FromStr;

/// Pretty print the set of roles from the database that match the provided criteria
///
/// # Arguments
/// * `client` - A Client instance used to connect to the database
/// * `cmd` - A PbFind enum instance used to extract the relevant commandline arguments
///
/// # Returns
/// * a Unit if Ok, or a boxed error if Err
pub fn find(client: Client, cmd: PbFind) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::Roles {
        role,
        category,
        order_by,
        ..
    } = cmd
    {
        //let (level, role, platform, site, mode) =
        //extract_coords(&level, &role, &platform, &site, &search_mode);
        let mut pb = PackratDb::new(client);
        let mut results = pb.find_all_roles();
        results
            .role_opt(role.as_ref().map(Deref::deref))
            .category_opt(category.as_ref().map(Deref::deref));
        if let Some(ref order) = order_by {
            let orders = order
                .split(",")
                .map(|x| {
                    OrderRoleBy::from_str(x).unwrap_or_else(|y| {
                        log::warn!("invalid order-by argument:'{}'. {}", x, y);
                        OrderRoleBy::Name
                    })
                })
                .collect::<Vec<OrderRoleBy>>();
            results.order_by(orders);
        }
        let results = results.query()?;
        // For now I do this. I need to add packge handling into the query
        // either by switching functions or handling the sql on this end

        let mut table = table!([bFg => "ROLE", "CATEGORY"]);
        for result in results {
            table.add_row(row![result.role, result.category]);
        }
        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    };

    Ok(())
}

/// Add one or more roles
pub fn add<'a>(tx: Transaction<'a>, cmd: PbAdd) -> Result<(), Box<dyn std::error::Error>> {
    if let PbAdd::Roles { mut names, .. } = cmd {
        let comment = "Auto Comment - Roles added";
        let username = whoami::username();
        let results = PackratDb::add_roles(tx)
            .roles(&mut names)
            .create()?
            .commit(&username, &comment)?;
        println!("{}", results);
    }
    Ok(())
}
