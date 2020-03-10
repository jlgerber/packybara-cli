# how to handle changes
We need a more flexible approach. Here are the relevant query examples

```

packrat=# select event_id,table_name,transaction_id as tx_id,action,action_tstamp_stm,row_data,changed_fields from audit.logged_actions where audit.logged_actions.transaction_id =13527;
 event_id | table_name | tx_id | action |       action_tstamp_stm       |                          row_data                          |    changed_fields     
----------+------------+-------+--------+-------------------------------+------------------------------------------------------------+-----------------------
     1594 | versionpin | 13527 | UPDATE | 2020-02-26 03:38:12.8342+00   | "id"=>"50", "coord"=>"50", "distribution"=>"362"           | "distribution"=>"366"
     1595 | versionpin | 13527 | UPDATE | 2020-02-26 03:38:12.840617+00 | "id"=>"52", "coord"=>"52", "distribution"=>"92"            | "distribution"=>"98"
     1596 | revision   | 13527 | INSERT | 2020-02-26 03:38:12.84273+00  | "id"=>"128", "author"=>"jgerber", "comment"=>"makin bacon" | 
(3 rows)


packrat=# select event_id,table_name,transaction_id as tx_id,action,action_tstamp_stm,row_data,changed_fields from audit.logged_actions where audit.logged_actions.transaction_id =13545;
 event_id | table_name  | tx_id | action |       action_tstamp_stm       |                                       row_data                                       | changed_fields 
----------+-------------+-------+--------+-------------------------------+--------------------------------------------------------------------------------------+----------------
     1597 | withpackage | 13545 | DELETE | 2020-02-27 05:45:00.955574+00 | "id"=>"406", "package"=>"atomic", "pinorder"=>"0", "versionpin"=>"24"                | 
     1598 | withpackage | 13545 | DELETE | 2020-02-27 05:45:00.955574+00 | "id"=>"407", "package"=>"atomic_assets_amtools", "pinorder"=>"1", "versionpin"=>"24" | 
     1599 | withpackage | 13545 | DELETE | 2020-02-27 05:45:00.955574+00 | "id"=>"408", "package"=>"vray", "pinorder"=>"2", "versionpin"=>"24"                  | 
     1600 | withpackage | 13545 | DELETE | 2020-02-27 05:45:00.955574+00 | "id"=>"409", "package"=>"wam", "pinorder"=>"3", "versionpin"=>"24"                   | 
     1601 | withpackage | 13545 | DELETE | 2020-02-27 05:45:00.955574+00 | "id"=>"410", "package"=>"wambase", "pinorder"=>"4", "versionpin"=>"24"               | 
     1602 | withpackage | 13545 | DELETE | 2020-02-27 05:45:00.955574+00 | "id"=>"411", "package"=>"race", "pinorder"=>"5", "versionpin"=>"24"                  | 
     1603 | withpackage | 13545 | DELETE | 2020-02-27 05:45:00.955574+00 | "id"=>"412", "package"=>"racetrack", "pinorder"=>"6", "versionpin"=>"24"             | 
     1604 | withpackage | 13545 | DELETE | 2020-02-27 05:45:00.955574+00 | "id"=>"413", "package"=>"babel", "pinorder"=>"7", "versionpin"=>"24"                 | 
     1605 | withpackage | 13545 | DELETE | 2020-02-27 05:45:00.955574+00 | "id"=>"414", "package"=>"houdini", "pinorder"=>"8", "versionpin"=>"24"               | 
     1606 | withpackage | 13545 | DELETE | 2020-02-27 05:45:00.955574+00 | "id"=>"415", "package"=>"snitcher", "pinorder"=>"9", "versionpin"=>"24"              | 
     1607 | withpackage | 13545 | DELETE | 2020-02-27 05:45:00.955574+00 | "id"=>"416", "package"=>"maya", "pinorder"=>"10", "versionpin"=>"24"                 | 
     1608 | withpackage | 13545 | INSERT | 2020-02-27 05:45:00.961884+00 | "id"=>"423", "package"=>"atomic", "pinorder"=>"0", "versionpin"=>"24"                | 
     1609 | withpackage | 13545 | INSERT | 2020-02-27 05:45:00.966016+00 | "id"=>"424", "package"=>"atomic_assets_amtools", "pinorder"=>"1", "versionpin"=>"24" | 
     1610 | withpackage | 13545 | INSERT | 2020-02-27 05:45:00.968009+00 | "id"=>"425", "package"=>"vray", "pinorder"=>"2", "versionpin"=>"24"                  | 
     1611 | withpackage | 13545 | INSERT | 2020-02-27 05:45:00.969929+00 | "id"=>"426", "package"=>"wam", "pinorder"=>"3", "versionpin"=>"24"                   | 
     1612 | withpackage | 13545 | INSERT | 2020-02-27 05:45:00.971887+00 | "id"=>"427", "package"=>"snitcher", "pinorder"=>"4", "versionpin"=>"24"              | 
     1613 | withpackage | 13545 | INSERT | 2020-02-27 05:45:00.973867+00 | "id"=>"428", "package"=>"wambase", "pinorder"=>"5", "versionpin"=>"24"               | 
     1614 | withpackage | 13545 | INSERT | 2020-02-27 05:45:00.975808+00 | "id"=>"429", "package"=>"race", "pinorder"=>"6", "versionpin"=>"24"                  | 
     1615 | withpackage | 13545 | INSERT | 2020-02-27 05:45:00.977692+00 | "id"=>"430", "package"=>"racetrack", "pinorder"=>"7", "versionpin"=>"24"             | 
     1616 | withpackage | 13545 | INSERT | 2020-02-27 05:45:00.979635+00 | "id"=>"431", "package"=>"babel", "pinorder"=>"8", "versionpin"=>"24"                 | 
     1617 | withpackage | 13545 | INSERT | 2020-02-27 05:45:00.981508+00 | "id"=>"432", "package"=>"houdini", "pinorder"=>"9", "versionpin"=>"24"               | 
     1618 | withpackage | 13545 | INSERT | 2020-02-27 05:45:00.983467+00 | "id"=>"433", "package"=>"maya", "pinorder"=>"10", "versionpin"=>"24"                 | 
     1619 | revision    | 13545 | INSERT | 2020-02-27 05:45:00.985388+00 | "id"=>"129", "author"=>"jgerber", "comment"=>"reorder"                               | 
(23 rows)
```