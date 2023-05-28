CREATE OR REPLACE FUNCTION calculate_user_count()
RETURNS NUMERIC AS $$
DECLARE
    result NUMERIC;
BEGIN
    SELECT (CASE WHEN relpages > 0 THEN (reltuples / relpages) * (pg_relation_size('users') / (current_setting('block_size')::integer))
                 ELSE (SELECT count(*) FROM users)
            END)
    INTO result
    FROM pg_class
    WHERE relname = 'users';

    RETURN result;
END;
$$ LANGUAGE plpgsql;
