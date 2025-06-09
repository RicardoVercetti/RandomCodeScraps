CREATE TABLE roles (
    role_id SERIAL PRIMARY KEY,
    role_name VARCHAR(50) UNIQUE NOT NULL
);


CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    full_name VARCHAR(100),
    role_id INT REFERENCES roles(role_id),
    institution_id INT,  -- Nullable
    branch_id INT,       -- Nullable
    department_id INT    -- Nullable
);

CREATE TABLE institutions (
    institution_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL
);

CREATE TABLE branches (
    branch_id SERIAL PRIMARY KEY,
    institution_id INT REFERENCES institutions(institution_id),
    name VARCHAR(100) NOT NULL
);

CREATE TABLE departments (
    department_id SERIAL PRIMARY KEY,
    branch_id INT REFERENCES branches(branch_id),
    name VARCHAR(100) NOT NULL
);

CREATE TABLE modules (
    module_id SERIAL PRIMARY KEY,
    module_name VARCHAR(100) UNIQUE NOT NULL
);

CREATE TABLE permissions (
    permission_id SERIAL PRIMARY KEY,
    permission_name VARCHAR(50) UNIQUE NOT NULL
);

CREATE TABLE department_module_permissions (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(user_id),  -- Dept Admin only
    module_id INT REFERENCES modules(module_id),
    permission_id INT REFERENCES permissions(permission_id),
    is_allowed BOOLEAN DEFAULT FALSE,
    UNIQUE(user_id, module_id, permission_id)
);

----------------------------------------------------------

INSERT INTO roles (role_name) VALUES
('Alpha Admin'),
('Institution Admin'),
('Branch Admin'),
('Department Admin');

INSERT INTO institutions (name) VALUES
('Bank A'),
('Bank B');

INSERT INTO branches (institution_id, name) VALUES
(1, 'Bank A - Main Branch'),
(1, 'Bank A - City Branch'),
(2, 'Bank B - Main Branch');

INSERT INTO departments (branch_id, name) VALUES
(1, 'Card Ops'),
(2, 'Customer Service'),
(3, 'Fraud Department');

-- Alpha Admin (full system access)
INSERT INTO users (username, password_hash, full_name, role_id) VALUES
('alpha_admin', 'hash_alpha', 'Alpha User', 1);

-- Institution Admin for Bank A
INSERT INTO users (username, password_hash, full_name, role_id, institution_id) VALUES
('inst_admin_a', 'hash_inst_a', 'Inst Admin A', 2, 1);

-- Branch Admin for Bank A - Main Branch
INSERT INTO users (username, password_hash, full_name, role_id, institution_id, branch_id) VALUES
('branch_admin_a1', 'hash_branch_a1', 'Branch Admin A1', 3, 1, 1);

-- Department Admin under Bank A - Main Branch
INSERT INTO users (username, password_hash, full_name, role_id, institution_id, branch_id, department_id) VALUES
('dept_admin_cardops', 'hash_dept1', 'Dept Admin CardOps', 4, 1, 1, 1);

INSERT INTO modules (module_name) VALUES
('Institution'),
('Branch'),
('Card Product'),
('Card Mailer'),
('Pin Mailer'),
('Keys'),
('HSM Config');

INSERT INTO permissions (permission_name) VALUES
('create'),
('read'),
('update'),
('delete'),
('import'),
('export');

-- Grant read and update on 'Card Mailer' (module_id = 4) to dept admin
INSERT INTO department_module_permissions (user_id, module_id, permission_id, is_allowed) VALUES
-- read (permission_id = 2)
(4, 4, 2, TRUE),
-- update (permission_id = 3)
(4, 4, 3, TRUE),

-- Deny delete (permission_id = 4)
(4, 4, 4, FALSE);


-- SELECT u.username, r.role_name, m.module_name, p.permission_name, dmp.is_allowed
-- FROM department_module_permissions dmp
-- JOIN users u ON dmp.user_id = u.user_id
-- JOIN roles r ON u.role_id = r.role_id
-- JOIN modules m ON dmp.module_id = m.module_id
-- JOIN permissions p ON dmp.permission_id = p.permission_id
-- ORDER BY u.username, m.module_name, p.permission_name;

SELECT * from 


SELECT 
    u.username,
    r.role_name,
    m.module_name,
    p.permission_name,
    
    -- Determine if the permission is allowed based on role
    CASE
        -- Alpha Admin gets all permissions
        WHEN r.role_name = 'Alpha Admin' THEN TRUE

        -- Institution Admin gets all permissions under their institution
        WHEN r.role_name = 'Institution Admin' AND 
             (u.institution_id = b.institution_id OR b.institution_id IS NULL) THEN TRUE

        -- Branch Admin gets all permissions under their branch
        WHEN r.role_name = 'Branch Admin' AND 
             (u.branch_id = b.branch_id OR b.branch_id IS NULL) THEN TRUE

        -- Department Admin checks department_module_permissions
        WHEN r.role_name = 'Department Admin' THEN COALESCE(dmp.is_allowed, FALSE)

        ELSE FALSE
    END AS is_allowed

FROM users u
JOIN roles r ON u.role_id = r.role_id
CROSS JOIN modules m
CROSS JOIN permissions p
LEFT JOIN branches b ON u.branch_id = b.branch_id
LEFT JOIN department_module_permissions dmp 
    ON u.user_id = dmp.user_id 
    AND dmp.module_id = m.module_id 
    AND dmp.permission_id = p.permission_id

WHERE u.username = 'inst_admin_a'  -- change this to target any user
ORDER BY m.module_name, p.permission_name;


