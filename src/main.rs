use mysql::{prelude::*, Row};
use mysql::{Pool};
use std::io;
// use chrono::{DateTime, Utc, NaiveDateTime, TimeZone};

// #[derive(Debug)]
// struct ApartmentMeter {
//     apartment_id: String,
//     meter_id: String
// }

// #[derive(Debug)]
// struct MeterData {
//     meter_id: String,
//     acitvity: String,
//     issue_date: String,
//     last_update: String
// }

// #[derive(Debug)]
// struct PaymentData {
//     apartment_id: String,
//     occupants: i32,
//     consumption: f64,
//     staff_id: String,
//     amount: f64,
//     payment_date: String
// }

#[derive(Debug)]
struct StaffData{
    staff_id: String,
    staff_name: String,
    staff_contact: i128
}

#[derive(Debug)]
struct DisplayRows{
    apartment_id: String,
    meter_id: String,
    acitvity: String,
    occupants: i32,
    amount_due: f64,
    staff_id: String,
}

#[derive(Debug)]
struct  ConsumptionGroups{
    consumption: f64,
    occupants: i32
}

fn main() {
    // Establishing a connection pool
    let db_url = "mysql://kelper:mysql@localhost:3306/DBMSIII";

    let pool = create_mysql_pool(db_url).expect("Failed to create MySQL pool");


    // uncomment the below line to test the function run -- execute_display
    // let result = execute_display(&pool, "amount", "desc");


    // uncomment the below cluster to test the function run -- display_Staff_info
        // let vec = display_staff_info(&pool, "SGCCID-2072");

        // match display_staff_info(&pool, "SGCCID-2072") {2
        //     Ok(result) => {
        //         if let Some(staff_at_index_0) = result.get(0) {
        //             // Access and use the struct at index 0
        //             println!("{:?}", staff_at_index_0);
        //         } else {
        //             println!("Vector is empty.");
        //         }
        //     }
        //     Err(err) => {
        //         eprintln!("Error: {}", err);
        //         // Handle the error case appropriately
        //     }
        // }

    // uncomment the below line to test the function run -- search_apartment_id
    // let result = search_apartment_id(&pool, "B13", "amount", "desc");


    // uncomment the below line to test the function run -- search_active
    // let result = search_active(&pool, true, "amount", "desc");


    let result = percentage_dist_water_consumption(&pool);

    for record in result.iter(){
        println!("{:?}\n", record);
    }

}

fn create_mysql_pool(db_url: &str) -> Result<Pool, mysql::Error> {
    Pool::new(db_url)
}

fn execute_display(pool: &Pool, order_param: String, order: String) -> Result<Vec<DisplayRows>, mysql::Error> {

    
    let query = format!(
        "select a.apartment_id, m.meter_id, m.active, p.Occupants, p.Amount, p.Staff_id from apartmentMeter a, meterData m, paymentData p where a.meter_id = m.meter_id and a.apartment_id = p.Apartment_id order by {} {};"
        , order_param, order);

    let mut conn = pool.get_conn()?;

    let result: Vec<DisplayRows> = conn.query_map(query, |row: mysql::Row| {

        DisplayRows {
            apartment_id: row.get("apartment_id").unwrap(),
            meter_id: row.get("meter_id").unwrap(),
            acitvity: row.get("active").unwrap(),
            occupants: row.get("Occupants").unwrap(),
            amount_due: row.get("Amount").unwrap(),
            staff_id: row.get("Staff_id").unwrap(),
        }
    }).expect("Failed to execute query");

    Ok(result)
}


fn display_staff_info(pool: &Pool, staff_id: &str) -> Result<Vec<StaffData>, mysql::Error> {
    let query: String = format!(
        "select * from staffData where Staff_id = '{}'",
        staff_id
    );

    let mut conn = pool.get_conn()?;

    let result: Vec<StaffData> = conn
        .query_map(query, |row: mysql::Row| {
            // Map the row to the StaffData struct
            StaffData {
                staff_id: row.get("Staff_id").unwrap(),
                staff_name: row.get("Staff_name").unwrap(),
                staff_contact: row.get("Staff_contact").unwrap(),
            }
        })?;

    Ok(result)
}

fn search_apartment_id(pool: &Pool, search: &str, order_param: &str, order: &str) -> Result<Vec<DisplayRows>, mysql::Error> {

    
    let query = format!(
        "select a.apartment_id, m.meter_id, m.active, p.Occupants, p.Amount, p.Staff_id from apartmentMeter a, meterData m, paymentData p where a.meter_id = m.meter_id and a.apartment_id = p.Apartment_id and a.apartment_id like \"%{}%\" order by {} {};"
        ,search , order_param, order);

    let mut conn = pool.get_conn()?;

    let result: Vec<DisplayRows> = conn.query_map(query, |row: mysql::Row| {
        DisplayRows {
            apartment_id: row.get("apartment_id").unwrap(),
            meter_id: row.get("meter_id").unwrap(),
            acitvity: row.get("active").unwrap(),
            occupants: row.get("Occupants").unwrap(),
            amount_due: row.get("Amount").unwrap(),
            staff_id: row.get("Staff_id").unwrap(),
        }
    }).expect("Failed to execute query");

    Ok(result)

}

fn search_meter_id(pool: &Pool, search: &str, order_param: &str, order: &str) -> Result<Vec<DisplayRows>, mysql::Error> {

    
    let query = format!(
        "select a.apartment_id, m.meter_id, m.active, p.Occupants, p.Amount, p.Staff_id from apartmentMeter a, meterData m, paymentData p where a.meter_id = m.meter_id and a.apartment_id = p.Apartment_id and m.meter_id like \"%{}%\" order by {} {};"
        ,search , order_param, order);

    let mut conn = pool.get_conn()?;

    // Execute the query
    let result: Vec<DisplayRows> = conn.query_map(query, |row: mysql::Row| {

        DisplayRows {
            apartment_id: row.get("apartment_id").unwrap(),
            meter_id: row.get("meter_id").unwrap(),
            acitvity: row.get("active").unwrap(),
            occupants: row.get("Occupants").unwrap(),
            amount_due: row.get("Amount").unwrap(),
            staff_id: row.get("Staff_id").unwrap(),
        }
    }).expect("Failed to execute query");

    Ok(result)

}

fn search_active(pool: &Pool, search: bool, order_param: &str, order: &str) -> Result<Vec<DisplayRows>, mysql::Error> {

    
    let query = format!(
        "select a.apartment_id, m.meter_id, m.active, p.Occupants, p.Amount, p.Staff_id from apartmentMeter a, meterData m, paymentData p where a.meter_id = m.meter_id and a.apartment_id = p.Apartment_id and m.active = \"{}\" order by {} {};"
        ,search , order_param, order);


    let mut conn = pool.get_conn()?;

    let result: Vec<DisplayRows> = conn.query_map(query, |row: mysql::Row| {

        DisplayRows {
            apartment_id: row.get("apartment_id").unwrap(),
            meter_id: row.get("meter_id").unwrap(),
            acitvity: row.get("active").unwrap(),
            occupants: row.get("Occupants").unwrap(),
            amount_due: row.get("Amount").unwrap(),
            staff_id: row.get("Staff_id").unwrap(),
        }
    }).expect("Failed to execute query");

    Ok(result)

}

fn search_occupants(pool: &Pool, lower_bound: i32, upper_bound: i32, order_param: &str, order: &str) -> Result<Vec<DisplayRows>, mysql::Error> {

    let query = format!(
        "select a.apartment_id, m.meter_id, m.active, p.Occupants, p.Amount, p.Staff_id from apartmentMeter a, meterData m, paymentData p where a.meter_id = m.meter_id and a.apartment_id = p.Apartment_id and p.Occupants >= {} and p.Occupants <= {} order by {} {};"
        ,lower_bound, upper_bound, order_param, order);        


    let mut conn = pool.get_conn()?;


    let result: Vec<DisplayRows> = conn.query_map(query, |row: mysql::Row| {

        DisplayRows {
            apartment_id: row.get("apartment_id").unwrap(),
            meter_id: row.get("meter_id").unwrap(),
            acitvity: row.get("active").unwrap(),
            occupants: row.get("Occupants").unwrap(),
            amount_due: row.get("Amount").unwrap(),
            staff_id: row.get("Staff_id").unwrap(),
        }
    }).expect("Failed to execute query");

    Ok(result)

}

fn search_staff_id(pool: &Pool, search: &str, order_param: &str, order: &str) -> Result<Vec<DisplayRows>, mysql::Error> {

    
    let query = format!(
        "select a.apartment_id, m.meter_id, m.active, p.Occupants, p.Amount, p.Staff_id from apartmentMeter a, meterData m, paymentData p where a.meter_id = m.meter_id and a.apartment_id = p.Apartment_id and s.staff_id like \"%{}%\" order by {} {};"
        ,search , order_param, order);


    let mut conn = pool.get_conn()?;


    let result: Vec<DisplayRows> = conn.query_map(query, |row: mysql::Row| {

        DisplayRows {
            apartment_id: row.get("apartment_id").unwrap(),
            meter_id: row.get("meter_id").unwrap(),
            acitvity: row.get("active").unwrap(),
            occupants: row.get("Occupants").unwrap(),
            amount_due: row.get("Amount").unwrap(),
            staff_id: row.get("Staff_id").unwrap(),
        }
    }).expect("Failed to execute query");

    Ok(result)

}

fn percentage_dist_occupants(pool: &Pool) -> Result<Vec<f64>, mysql::Error> {
    let query = "SELECT Occupants FROM paymentData";

    let mut conn = pool.get_conn()?;


    let result: Vec<i32> = conn
        .query_map(query, |row: Row| {
            let occupants: i32 = row.get("Occupants").unwrap();
            occupants
        })
        .map_err(|err| {
            eprintln!("Error executing query: {}", err);
            err
        })?;

    let mut dist: Vec<f64> = vec![0.0, 0.0, 0.0, 0.0];

    let one: &i32 = &1;
    let three: &i32 = &3;
    // let eight: i32 = 8;

    for rec in result.iter() {
        if rec == one {
            *dist.get_mut(0).unwrap() += 1.0;
        } else if rec > one && rec <= three {
            *dist.get_mut(1).unwrap() += 1.0;
        } else if rec > three{
            *dist.get_mut(2).unwrap() += 1.0;
        } else {
            *dist.get_mut(3).unwrap() += 1.0;
        }
    }
    *dist.get_mut(0).unwrap() /= 208.0;
    *dist.get_mut(1).unwrap() /= 208.0;
    *dist.get_mut(2).unwrap() /= 208.0;
    *dist.get_mut(3).unwrap() /= 208.0;

    println!("{}", *dist.get_mut(0).unwrap()+*dist.get_mut(1).unwrap()+*dist.get_mut(2).unwrap()+*dist.get_mut(3).unwrap());

    Ok(dist)


}



fn percentage_dist_water_consumption(pool: &Pool) -> Result<Vec<f64>, mysql::Error> {
    let query = "select sum(Consumption_level), Occupants from paymentData group by Occupants;";


    let mut conn = pool.get_conn()?;


    let result: Vec<ConsumptionGroups> = conn.query_map(query, |row: mysql::Row| {

        ConsumptionGroups {
            consumption: row.get("sum(Consumption_level)").unwrap(),
            Occupants: row.get("Occupants").unwrap(),
           
        }
    }).expect("Failed to execute query");

    let mut dist: Vec<f64> = vec![0.0, 0.0, 0.0, 0.0];

    let one: i32 = 1;
    let three: i32 = 3;
    // let eight: i32 = 8;

    for rec in result.iter() {
        if rec.Occupants == one {
            *dist.get_mut(0).unwrap() += rec.consumption;
        } else if rec.Occupants > one && rec.Occupants <= three {
            *dist.get_mut(1).unwrap() += rec.consumption;
        } else if rec.Occupants > three{
            *dist.get_mut(2).unwrap() += rec.consumption;
        } else {
            *dist.get_mut(3).unwrap() += rec.consumption;
        }
    }

    println!("{}", *dist.get_mut(0).unwrap()+*dist.get_mut(1).unwrap()+*dist.get_mut(2).unwrap()+*dist.get_mut(3).unwrap());

    Ok(dist)


}