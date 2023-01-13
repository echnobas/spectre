CREATE OR REPLACE PROCEDURE register_group(d_id text)
LANGUAGE plpgsql as
$$
  BEGIN
        EXECUTE 'CREATE SCHEMA ' || quote_ident('data_' || $1) || ' CREATE TABLE members (rbx_id bigint PRIMARY KEY, d_id bigint, xp bigint)
        CREATE TABLE roles (role_id int PRIMARY KEY, xp_threshold bigint, locked boolean);';
  END
$$;

CREATE OR REPLACE PROCEDURE ADD_XP(d_id text, rbx_id bigint, xp bigint)
LANGUAGE plpgsql as
$$
BEGIN
        EXECUTE 'INSERT INTO ' || quote_ident('data_' || $1) || '.members (rbx_id, d_id, xp) VALUES(' || $2 || ', NULL, ' || $3 || ') ON CONFLICT (rbx_id) DO UPDATE SET xp = members.xp+' || $3 || ' WHERE members.rbx_id = ' || $2 || ';';
END
$$;