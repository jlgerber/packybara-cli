/*******************************************************
 * Copyright (C) 2019,2020 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use super::args::PbFind;
use packybara::db::traits::*;
use packybara::packrat::{Client, PackratDb};
use packybara::SearchMode;
use prettytable::{cell, format, row, table};
use std::ops::Deref;

/// Pretty print the set of package coordinates from the database that match the provided criteria
///
/// # Arguments
/// * `client` - A Client instance used to connect to the database
/// * `cmd` - A PbFind enum instance used to extract the relevant commandline arguments
///
/// # Returns
/// * a Unit if Ok, or a boxed error if Err
pub fn find(client: Client, cmd: PbFind) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::PkgCoords {
        package,
        level,
        role,
        platform,
        site,
        search_mode,
        order_by,
        ..
    } = cmd
    {
        let mut pb = PackratDb::new(client);
        let mut results = pb.find_pkgcoords();
        results
            .package_opt(package.as_ref().map(Deref::deref))
            .role_opt(role.as_ref().map(Deref::deref))
            .level_opt(level.as_ref().map(Deref::deref))
            .platform_opt(platform.as_ref().map(Deref::deref))
            .site_opt(site.as_ref().map(Deref::deref))
            .order_by_opt(order_by.as_ref().map(Deref::deref));
        if let Some(ref mode) = search_mode {
            results.search_mode(SearchMode::try_from_str(mode)?);
        }
        let results = results.query()?;
        // For now I do this. I need to add packge handling into the query
        // either by switching functions or handling the sql on this end
        let mut table = table!([bFg => "ID","PACKAGE", "ROLE", "LEVEL", "PLATFORM", "SITE"]);
        for result in results {
            table.add_row(row![
                result.id,
                result.package,
                result.role,
                result.level,
                result.platform,
                result.site
            ]);
        }
        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    };

    Ok(())
}
