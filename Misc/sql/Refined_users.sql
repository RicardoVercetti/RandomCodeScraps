CREATE TABLE roles (
    role_id SERIAL PRIMARY KEY,
    role_name TEXT UNIQUE NOT NULL
);


CREATE TABLE modules (
    module_id SERIAL PRIMARY KEY,
    module_name TEXT UNIQUE NOT NULL
);


CREATE TABLE permissions (
    permission_id SERIAL PRIMARY KEY,
    permission_name TEXT UNIQUE NOT NULL
);


CREATE TABLE role_module_permissions (
    role_id INT REFERENCES roles(role_id),
    module_id INT REFERENCES modules(module_id),    -- same module differs for different user on a different level
    permission_id INT REFERENCES permissions(permission_id),
    is_allowed BOOLEAN DEFAULT FALSE,
    PRIMARY KEY (role_id, module_id, permission_id)
);


CREATE TABLE userz (
    user_id SERIAL PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    full_name TEXT,
    role_id INT REFERENCES roles(role_id),
    institution_id INT,
    branch_id INT,
    department_id INT
);

CREATE TABLE levels(
    level_id SERIAL PRIMARY KEY,
    level_name TEXT
);

create TABLE role_lvl (
    role_id INT REFERENCES roles(role_id),
    level_id INT REFERENCES levels(level_id)
);



---------------------------------

-- Insert base permissions
INSERT INTO permissions (permission_name) VALUES 
('Read'), ('Create/Update/Delete'), ('Import'), ('Export'), ('Maker'), ('Checker');

-- Insert modules
INSERT INTO modules (module_name) VALUES 
('Institution'), ('Branch'), ('Card Product'), ('Card Mailer'),
('Pin Mailer'), ('Keys'), ('HSM Config'), ('Users');

-- Insert role
-- predefined roles
INSERT INTO roles (role_name) VALUES ('Alpha Admin');  -- has all controls by default, if new Institution gets added

INSERT INTO roles (role_name) VALUES ('Institution Admin');  -- Institution admin has all module permissions 

INSERT INTO levels (level_name) VALUES ('AA'), ('IA'), ('BA'), ('CU');

INSERT INTO role_lvl(role_id, level_id) VALUES (1, 1), (2, 2);



-- customizable roles


-- Full permissions for Institution Admin
INSERT INTO role_module_permissions (role_id, module_id, permission_id, is_allowed)
SELECT
    r.role_id,
    m.module_id,
    p.permission_id,
    TRUE
FROM roles r
JOIN modules m ON TRUE
JOIN permissions p ON TRUE
WHERE r.role_name = 'Institution Admin';


-- Grant all permissions on Users module
INSERT INTO role_module_permissions (role_id, module_id, permission_id, is_allowed)
SELECT
    r.role_id,
    m.module_id,
    p.permission_id,
    TRUE
FROM roles r
JOIN modules m ON m.module_name = 'Users'
JOIN permissions p ON TRUE
WHERE r.role_name = 'Alpha Admin';

-- Grant only create and read on Institution module
INSERT INTO role_module_permissions (role_id, module_id, permission_id, is_allowed)
SELECT
    r.role_id,
    m.module_id,
    p.permission_id,
    TRUE
FROM roles r
JOIN modules m ON m.module_name = 'Institution'
JOIN permissions p ON p.permission_name IN ('create/update/delete')
WHERE r.role_name = 'Alpha Admin';

-- create user with role_id of Alpha Admin
INSERT INTO userz (username, password_hash, full_name, role_id)
VALUES ('The Owner', 'hashed_pass_1', 'Martin Stein', 
        (SELECT role_id FROM roles WHERE role_name = 'Alpha Admin'));
        
-- user with Institution admin role
INSERT INTO userz (username, password_hash, full_name, role_id)
VALUES ('Pekoms', 'hashed_pass_2', 'Pekoms B M', 
        (SELECT role_id FROM roles WHERE role_name = 'Institution Admin'));



-- a branch level role



-- SELECT 
--     u.full_name AS user,
--     r.role_name,
--     m.module_name,
--     p.permission_name,
--     rmp.is_allowed
-- FROM users u
-- JOIN roles r ON u.role_id = r.role_id
-- JOIN role_module_permissions rmp ON r.role_id = rmp.role_id
-- JOIN modules m ON rmp.module_id = m.module_id
-- JOIN permissions p ON rmp.permission_id = p.permission_id
-- WHERE u.username = 'The Owner'
-- ORDER BY m.module_name, p.permission_name;

-- SELECT 
--     u.full_name AS user,
--     r.role_name,
--     m.module_name,
--     p.permission_name,
--     rmp.is_allowed
-- FROM users u
-- JOIN roles r ON u.role_id = r.role_id
-- JOIN role_module_permissions rmp ON r.role_id = rmp.role_id
-- JOIN modules m ON rmp.module_id = m.module_id
-- JOIN permissions p ON rmp.permission_id = p.permission_id
-- WHERE u.username = 'Pekoms'
-- ORDER BY m.module_name, p.permission_name;


-- SELECT * FROM role_module_permissions;








SELECT r.role_name, ll.level_name FROM roles r join role_lvl as l on r.role_id = l.role_id 
      join levels as ll on l.level_id = ll.level_id;


