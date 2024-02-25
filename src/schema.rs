// @generated automatically by Diesel CLI.

diesel::table! {
    commissions (id) {
        id -> Nullable<Integer>,
        variable_rate -> Nullable<Double>,
        commission_amt -> Nullable<Double>,
        attainment -> Nullable<Double>,
        variable_comp -> Nullable<Double>,
        quota -> Nullable<Double>,
        deal_revenue -> Nullable<Double>,
        created_at -> Nullable<Timestamp>,
    }
}
