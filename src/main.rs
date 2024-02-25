use rocket::{get, post, routes, launch};
use rocket::response::Redirect;
use rocket::http::Status;
use rocket_dyn_templates::Template;
use rocket::form::{Form, FromForm};
use serde::{Deserialize, Serialize};
use crate::db::DbConn;
use rocket_sync_db_pools::*; // Required for Rocket's database pooling.
use diesel::{Insertable,Queryable};
use diesel::prelude::*;

mod schema;
mod db;

#[derive(Serialize, Deserialize, Queryable)]
struct Commission {
    id: Option<i32>,
    variable_rate: Option<f64>,
    commission_amt: Option<f64>,
    attainment: Option<f64>,
    variable_comp: Option<f64>,
    quota: Option<f64>,
    deal_revenue: Option<f64>,
    created_at: Option<chrono::NaiveDateTime>,
}

//use crate::schema::commissions::dsl::commissions;
use crate::schema::commissions;


#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = commissions)]
struct NewCommission {
    variable_rate: f64,
    commission_amt: f64,
    attainment: f64,
    variable_comp: f64,
    quota: f64,
    deal_revenue: f64,
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
            .load::<Commission>(conn)
            .expect("Error loading commissions");

        println!("Retrieved commissions from the database"); // Simple logging

        // https://api.rocket.rs/master/rocket_dyn_templates
        Ok(Template::render("index", &TemplateContext { commissions: result }))
    }).await
    .map_err(|_err: diesel::result::Error| "Error retrieving commissions: {}")

}

#[post("/", data = "<commission_form>")]
async fn submit_commission(db: DbConn, commission_form: Form<CommissionForm>) -> Result<Redirect, Status> {
    db.run(move |conn| {

        let form = commission_form.into_inner(); // Add this line

        let new_commission = NewCommission {
            variable_rate: form.variable_comp / form.quota, 
            commission_amt: form.variable_comp / form.quota  * form.deal_revenue,
            attainment: (form.deal_revenue / form.quota) * 100.0,  
            created_at: chrono::Local::now().naive_local(), // Or other logic here
            quota: form.quota,
            deal_revenue: form.deal_revenue,
            variable_comp: form.variable_comp
        };        

        // Log message
        println!("Attempting to insert commission into database");

        // Insert into database
        use crate::schema::commissions::dsl::*; // Adjust according to your schema

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
    let _ = db.run(move |conn| {
        let form = delete_form.into_inner();

        for commission_id in form.commission_ids {
            println!("Attempting to delete commission from database with ID: {}", commission_id);
            use crate::schema::commissions::dsl::{commissions, id};

            match diesel::delete(commissions.filter(id.eq(commission_id))).execute(conn) {
                Ok(_) => println!("Successfully deleted commission from database with ID: {}", commission_id),
                Err(err) => {
                    println!("Error deleting commission from database: {}", err);
                    return Err(From::from(err)); // Manual conversion
                }
            };                       
        }
        Ok(())
    }).await
    .map_err(|err: diesel::result::Error| { // Map the Diesel error
        println!("Error deleting commissions: {}", err);
        Status::InternalServerError  // Return a Rocket Status code
    });

    Ok(Redirect::to("/"))
}

#[launch]
fn rocket() -> _ {
    // Build Rocket instance
    let rocket = rocket::build()
        .attach(Template::fairing()) // Use the existing fairing
        .attach(db::DbConn::fairing())
        .mount("/", routes![home, submit_commission, delete_commission])
        .mount("/static", rocket::fs::FileServer::from("static/"));

    rocket
}
