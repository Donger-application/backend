create table if not exists "group"
(
    id        serial
        primary key,
    name      varchar not null,
    is_public boolean not null
);

alter table "group"
    owner to postgres;

create table if not exists role
(
    id   serial
        primary key,
    name varchar not null
);

alter table role
    owner to postgres;

create table if not exists "user"
(
    id              serial
        primary key,
    name            varchar               not null,
    balance         integer               not null,
    is_active       boolean               not null,
    role_id         integer               not null
        references role,
    group_id        integer               not null
        references "group",
    password        varchar               not null,
    email           varchar               not null
        unique,
    email_confirmed boolean default false not null,
    user_display_id varchar               not null
        unique
);

alter table "user"
    owner to postgres;

create table if not exists product
(
    id       serial
        primary key,
    name     varchar not null,
    group_id integer not null
        references "group"
);

alter table product
    owner to postgres;

create table if not exists stock
(
    id         serial
        primary key,
    price      bigint  not null,
    consumed   boolean not null,
    product_id integer not null
        unique
        references product
);

alter table stock
    owner to postgres;

create table if not exists meal
(
    id serial
        primary key
);

alter table meal
    owner to postgres;

create table if not exists meal_product
(
    meal_id    integer not null
        constraint fk_meal_product_meal
            references meal
            on update cascade on delete cascade,
    product_id integer not null
        constraint fk_meal_product_product
            references product
            on update cascade on delete cascade,
    primary key (meal_id, product_id)
);

alter table meal_product
    owner to postgres;

create table if not exists supplier
(
    id      serial
        primary key,
    balance integer not null,
    user_id integer not null
        unique
        references "user"
);

alter table supplier
    owner to postgres;

create table if not exists customer
(
    id      serial
        primary key,
    user_id integer not null
        references "user",
    balance integer not null
);

alter table customer
    owner to postgres;

create table if not exists invoice
(
    id                     serial
        primary key,
    price                  bigint    not null,
    is_deleted             boolean   not null,
    deleted_by             integer   not null,
    created_date           timestamp not null,
    last_modification_date timestamp not null,
    meal_id                integer   not null
        unique
        references meal,
    group_id               integer   not null
        unique
        references "group",
    supplier_id            integer   not null
        unique
        references supplier
);

alter table invoice
    owner to postgres;

create table if not exists invoice_details
(
    id         serial
        primary key,
    invoice_id integer not null
        references invoice,
    stock_id   integer not null
        references stock
);

alter table invoice_details
    owner to postgres;

create table if not exists "order"
(
    id                     serial
        primary key,
    is_deleted             boolean   not null,
    deleted_by             integer   not null,
    created_date           timestamp not null,
    last_modification_date timestamp not null,
    group_id               integer   not null
        references "group"
);

alter table "order"
    owner to postgres;

create table if not exists order_customer
(
    order_id    integer not null
        references "order",
    customer_id integer not null
        references customer,
    primary key (order_id, customer_id)
);

alter table order_customer
    owner to postgres;

create table if not exists order_details
(
    id       serial
        primary key,
    order_id integer not null
        references "order",
    stock_id integer not null
        references stock
);

alter table order_details
    owner to postgres;

create table if not exists system_log
(
    id               serial
        primary key,
    transaction_type varchar   not null,
    description      varchar   not null,
    date             timestamp not null,
    user_id          integer   not null
        references "user",
    group_id         integer   not null
        references "group"
);

alter table system_log
    owner to postgres;

create table if not exists active_session
(
    id       serial
        primary key,
    user_id  integer not null
        unique
        references "user",
    group_id integer not null
        references "group"
);

alter table active_session
    owner to postgres;


-- removing unique from indexes
ALTER TABLE invoice DROP CONSTRAINT invoice_meal_id_key;
ALTER TABLE invoice DROP CONSTRAINT invoice_group_id_key;
ALTER TABLE invoice DROP CONSTRAINT invoice_supplier_id_key;

DROP INDEX IF EXISTS invoice_meal_id_key;
DROP INDEX IF EXISTS invoice_group_id_key;
DROP INDEX IF EXISTS invoice_supplier_id_key;

CREATE INDEX invoice_meal_id_idx ON invoice(meal_id);
CREATE INDEX invoice_group_id_idx ON invoice(group_id);
CREATE INDEX invoice_supplier_id_idx ON invoice(supplier_id);