-- Your SQL goes here
CREATE TABLE commissions (
    id INTEGER PRIMARY KEY,
    variable_rate DOUBLE,
    commission_amt DOUBLE,
    attainment DOUBLE,
    variable_comp DOUBLE,
    quota DOUBLE,
    created_at TIMESTAMP
);
