-- =====================================
-- Groups
-- =====================================
INSERT INTO "group" (name, is_public)
VALUES ('Admins', false),
    ('Customers', true),
    ('Suppliers', true);
-- =====================================
-- Roles
-- =====================================
INSERT INTO role (name)
VALUES ('Admin'),
    ('User'),
    ('Manager');
-- =====================================
-- Users
-- =====================================
INSERT INTO "user" (
        name,
        balance,
        is_active,
        role_id,
        group_id,
        password,
        email,
        email_confirmed,
        user_display_id
    )
VALUES (
        'Alice',
        1000,
        true,
        1,
        1,
        'password123',
        'alice@example.com',
        true,
        'ALC01'
    ),
    (
        'Bob',
        500,
        true,
        2,
        2,
        'password123',
        'bob@example.com',
        false,
        'BOB01'
    ),
    (
        'Charlie',
        -750,
        true,
        3,
        1,
        'password123',
        'charlie@example.com',
        true,
        'CHR01'
    ),
    (
        'James',
        12500,
        true,
        3,
        1,
        'password123',
        'James@example.com',
        true,
        'JMS01'
    ),
    (
        'Roland',
        -3580,
        true,
        3,
        1,
        'password123',
        'Roland@example.com',
        true,
        'RLD01'
    ),
    (
        'Emilie',
        620,
        false,
        3,
        1,
        'password123',
        'Emillie@example.com',
        true,
        'EML01'
    ),
    (
        'Waylen',
        25,
        true,
        3,
        1,
        'password123',
        'Waylen@example.com',
        true,
        'WLN01'
    );
-- =====================================
-- Products
-- =====================================
INSERT INTO product (name, group_id)
VALUES ('Product A', 1),
    ('Product B', 2),
    ('Product C', 3),
    ('Product D', 1),
    ('Product E', 1),
    ('Product F', 1),
    ('Product G', 1),
    ('Product H', 1),
    ('Product I', 1),
    ('Product J', 1),
    ('Product K', 1);
-- =====================================
-- Stock
-- =====================================
INSERT INTO stock (price, consumed, product_id)
VALUES (100, false, 1),
    (200, false, 2),
    (800, false, 11),
    (270, false, 9),
    (650, false, 3),
    (430, true, 1),
    (111, false, 7),
    (302, false, 6),
    (901, false, 8);
-- =====================================
-- Meals
-- =====================================
INSERT INTO meal DEFAULT
VALUES;
-- Meal 1
INSERT INTO meal DEFAULT
VALUES;
-- Meal 2
-- =====================================
-- Meal-Product links
-- =====================================
INSERT INTO meal_product (meal_id, product_id)
VALUES (1, 1),
    (1, 2),
    (2, 2),
    (2, 3);
-- =====================================
-- Suppliers
-- =====================================
INSERT INTO supplier (balance, user_id)
VALUES (5000, 3),
        (3000, 2),
        (4250, 5),
        (7750, 6),
        (660, 7);
-- =====================================
-- Customers
-- =====================================
INSERT INTO customer (user_id, balance)
VALUES (2, 1000);
-- =====================================
-- Invoices
-- =====================================
INSERT INTO invoice ( price, is_deleted, deleted_by, created_date, last_modification_date, meal_id, group_id, supplier_id)
    VALUES (300, false, 0, NOW (), NOW (), 1, 1, 1),
        (400, false, 0, NOW (), NOW (), 1, 1, 2),
        (750, false, 0, NOW (), NOW (), 2, 1, 3),
        (100, true, 1, NOW (), NOW (), 2, 1, 1),
        (423, false, 0, NOW (), NOW (), 2, 1, 1);
-- =====================================
-- Invoice Details
-- =====================================
INSERT INTO invoice_details (invoice_id, stock_id)
VALUES (1, 1),
    (1, 2),
    (1, 7),
    (2, 13),
    (2, 8),
    (2, 9),
    (3, 1),
    (4, 2),
    (4, 3),
    (4, 10);
-- =====================================
-- Orders
-- =====================================
INSERT INTO "order" (
        is_deleted,
        deleted_by,
        created_date,
        last_modification_date,
        group_id
    )
VALUES (false, 0, NOW (), NOW (), 2);
-- =====================================
-- Order-Customer links
-- =====================================
INSERT INTO order_customer (order_id, customer_id)
VALUES (1, 1);
-- =====================================
-- Order Details
-- =====================================
INSERT INTO order_details (order_id, stock_id)
VALUES (1, 2),
    (1, 3);
-- =====================================
-- System Log
-- =====================================
INSERT INTO system_log (
        transaction_type,
        description,
        date,
        user_id,
        group_id
    )
VALUES ('CREATE', 'Created initial data', NOW (), 1, 1);
-- =====================================
-- Active Session
-- =====================================
INSERT INTO active_session (user_id, group_id)
VALUES (1, 1);
-- Reset sequences for all tables
ALTER SEQUENCE "group_id_seq" RESTART WITH 1;
ALTER SEQUENCE role_id_seq RESTART WITH 1;
ALTER SEQUENCE "user_id_seq" RESTART WITH 1;
ALTER SEQUENCE product_id_seq RESTART WITH 1;
ALTER SEQUENCE stock_id_seq RESTART WITH 1;
ALTER SEQUENCE meal_id_seq RESTART WITH 1;
ALTER SEQUENCE meal_product_meal_id_seq RESTART WITH 1;
-- if needed
ALTER SEQUENCE supplier_id_seq RESTART WITH 1;
ALTER SEQUENCE customer_id_seq RESTART WITH 1;
ALTER SEQUENCE invoice_id_seq RESTART WITH 1;
ALTER SEQUENCE invoice_details_id_seq RESTART WITH 1;
ALTER SEQUENCE "order_id_seq" RESTART WITH 1;
ALTER SEQUENCE order_details_id_seq RESTART WITH 1;
ALTER SEQUENCE system_log_id_seq RESTART WITH 1;
ALTER SEQUENCE active_session_id_seq RESTART WITH 1;
-- remove unique inside stock indexes
ALTER TABLE stock DROP CONSTRAINT stock_product_id_key;
create index stock_product_id_key on stock (product_id);
-- remove unique inside invoice details indexes
ALTER TABLE invoice_details DROP CONSTRAINT invoice_details_invoice_id_fkey;
create index invoice_details_invoice_id_fkey on stock (product_id);