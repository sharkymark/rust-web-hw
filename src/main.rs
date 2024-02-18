use rocket::{get, post, routes, launch};
use rocket::response::Redirect;
use rocket::http::Status;
use rocket_dyn_templates::Template;
use rocket::form::{Form, FromForm};
use serde::{Deserialize, Serialize};
use crate::db::DbConn;
use rocket_sync_db_pools::*; // Required for Rocket's database pooling.
use diesel::Insertable;
use diesel::Queryable;
use crate::schema::commissions::dsl::commissions;

mod schema;
mod db;

#[derive(Serialize, Deserialize, Queryable)]
struct Commission {
    id: i32,
    variable_rate: f64,
    commission_amt: f64,
    attainment: f64,
    variable_comp: f64,
    quota: f64,
    created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize,Insertable)]
#[table_name = "commissions"]
struct NewCommission {
    variable_rate: f64,
    commission_amt: f64,
    attainment: f64,
    variable_comp: f64,
    quota: f64,
    created_at: chrono::NaiveDateTime,
}

#[derive(FromForm)]
struct CommissionForm {
    quota: f64,
    variable_comp: f64,
    deal_revenue: f64,
}

#[derive(FromForm)]
struct DeleteForm {
    commission_ids: Vec<i32>, // Assuming commission IDs are i32
}




#[derive(Serialize)]
struct TemplateContext {
    commissions: Vec<Commission>,
    // Additional fields for the template can be added here
}

#[get("/")]
async fn home(db: DbConn) -> Result<Template, &'static str> {
    db.run(move |conn| {
        use crate::schema::commissions::dsl::*; // Adjust the path according to your schema module

        let result = commissions
            .load::<Commission>(&conn)
            .expect("Error loading commissions");

        println!("Retrieved commissions from the database"); // Simple logging

        Ok(Template::render("index", &TemplateContext { commissions: result }))
    }).await
    .map_err(|_| "Error retrieving commissions")
}

#[post("/", data = "<commission_form>")]
async fn submit_commission(db: DbConn, commission_form: Form<CommissionForm>) -> Result<Redirect, Status> {
    db.run(move |conn| {
        let form = commission_form.into_inner();
        let variable_rate = form.variable_comp / form.quota;
        let commission_amt = variable_rate * form.deal_revenue;
        let attainment = (form.deal_revenue / form.quota) * 100.0;

        // Log message
        println!("Attempting to insert commission into database");

        // Insert into database
        use crate::schema::commissions::dsl::*; // Adjust according to your schema

        let new_commission = NewCommission {
            variable_rate,
            commission_amt,
            attainment,
            variable_comp: form.variable_comp,
            quota: form.quota,
            created_at: chrono::Local::now().naive_local(),
        };

        // Using Diesel to insert the new commission
        diesel::insert_into(commissions)
            .values(&new_commission)
            .execute(conn)
            .expect("Error inserting commission into database");

        println!("Successfully inserted commission into database");
    }).await;

    Ok(Redirect::to("/"))
}

#[post("/delete-row", data = "<delete_form>")]
async fn delete_commission(db: DbConn, delete_form: Form<DeleteForm>) -> Result<Redirect, Status> {
    db.run(move |conn| {
        let form = delete_form.into_inner();

        for commission_id in form.commission_ids {
            println!("Attempting to delete commission from database with ID: {}", commission_id);
            use crate::schema::commissions::dsl::{commissions, id};

            match diesel::delete(commissions.filter(id.eq(commission_id))).execute(conn) {
                Ok(_) => println!("Successfully deleted commission from database with ID: {}", commission_id),
                Err(err) => {
                    println!("Error deleting commission from database: {}", err);
                    return Err(Status::InternalServerError);
                }
            };
        }
        Ok(())
    }).await;

    Ok(Redirect::to("/"))
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build()
        .attach(Template::fairing())
        .attach(db::DbConn::fairing())
        .mount("/", routes![home, submit_commission, delete_commission])
        .mount("/static", rocket::fs::FileServer::from("static/"));

    rocket
}
