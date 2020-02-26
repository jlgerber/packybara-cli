```
select 
    transaction_id as tx_id,
    client_query,
    action,
    row_data->'id' as id, 
    row_data-> 'coord' as coord, 
    row_data->'distribution' as dist,
    case when action = 'UPDATE' then
        changed_fields -> 'distribution'
    else 
        ''
    end as new_dist
from 
    audit.logged_actions 
where 
    audit.logged_actions.transaction_id=13527
and 
    audit.logged_actions.client_query <> 'INSERT INTO REVISION (author, comment) VALUES ($1, $2)';
```


```
WITH auditvals AS 
    (
        SELECT 
            row_data->'id' AS id,
            row_data->'coord' AS coord, 
            row_data->'distribution' AS old_dist,
            audit.logged_actions.changed_fields->'distribution' AS new_dist,
            audit.logged_actions.transaction_id,
            audit.logged_actions.action
        FROM 
            audit.logged_actions
        WHERE 
            table_name='versionpin' AND 
            row_data is not null AND 
			audit.logged_actions.transaction_id = tx_id
    ) 
SELECT 
    auditvals.id::integer AS id,
    auditvals.action,
    auditvals.transaction_id::BIGINT,
    pkgcoord.role_name, 
    pkgcoord.level_name,
    pkgcoord.site_name,
    pkgcoord.platform_name,
	pkgcoord.package,
    CASE 
        WHEN auditvals.action = 'UPDATE' THEN distribution.name
        ELSE ''
    END AS old,
    distribution2.name AS new
FROM 
    pkgcoord_view as pkgcoord
INNER JOIN 
    auditvals 
ON 
    auditvals.coord::integer = pkgcoord.pkgcoord_id 
INNER JOIN 
    distribution_view AS distribution 
ON 
    auditvals.old_dist::INTEGER = distribution.distribution_id
INNER JOIN 
    distribution_view as distribution2
ON 
   CASE WHEN auditvals.action = 'UPDATE' THEN auditvals.new_dist::INTEGER 
    ELSE auditvals.old_dist::INTEGER
    END = distribution2.distribution_id;
```

WITH auditvals AS 
    (
select 
    transaction_id as tx_id,
    client_query,
    action,
    row_data->'id' as id, 
    row_data-> 'coord' as coord, 
    row_data->'distribution' as dist,
    case when action = 'UPDATE' then
        changed_fields -> 'distribution'
    else 
        ''
    end as new_dist
from 
    audit.logged_actions 
where 
    table_name='versionpin' AND 
    row_data is not null AND 
    audit.logged_actions.transaction_id=13521 AND 
    audit.logged_actions.client_query <> 'INSERT INTO REVISION (author, comment) VALUES ($1, $2)'
    )
    SELECT 
    auditvals.id::integer AS id,
    auditvals.action,
    auditvals.tx_id::BIGINT,
    pkgcoord.role_name, 
    pkgcoord.level_name,
    pkgcoord.site_name,
    pkgcoord.platform_name,
	pkgcoord.package,
    CASE 
        WHEN auditvals.action = 'UPDATE' THEN distribution.name
        ELSE ''
    END AS old,
    distribution2.name AS new
FROM 
    pkgcoord_view as pkgcoord
INNER JOIN 
    auditvals 
ON 
    auditvals.coord::integer = pkgcoord.pkgcoord_id 
INNER JOIN 
    distribution_view AS distribution 
ON 
    auditvals.dist::INTEGER = distribution.distribution_id
INNER JOIN 
    distribution_view as distribution2
ON 
   CASE WHEN auditvals.action = 'UPDATE' THEN auditvals.new_dist::INTEGER 
    ELSE auditvals.dist::INTEGER
    END = distribution2.distribution_id;