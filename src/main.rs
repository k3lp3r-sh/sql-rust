// main.rs

use mysql::prelude::*;
use mysql::{Pool};

fn main() {
    // MySQL database connection parameters
    let db_url = "mysql://kelper:mysql@localhost:3306/DBMSIII";

    // Establish a connection pool
    let pool = create_mysql_pool(db_url).expect("Failed to create MySQL pool");

    // Example query
    let result = execute_query(&pool);

    match result {
        Ok(rows_affected) => {
            println!("Query executed successfully. Rows affected: {}", rows_affected);
        }
        Err(err) => {
            eprintln!("Error executing query: {}", err);
        }
    }
}

fn create_mysql_pool(db_url: &str) -> Result<Pool, mysql::Error> {
    Pool::new(db_url)
}

fn execute_query(pool: &Pool) -> Result<usize, mysql::Error> {
    // Example query: Inserting a row into a table
    let query = "INSERT INTO your_table_name (column1, column2) VALUES (\"A\", 2)";

    // Parameters for the query
    // let params = params! {
    //     "value1" => "some_value",
    //     "value2" => 42,
    // };

    // Get a connection from the pool
    let mut conn = pool.get_conn()?;

    // Execute the query
    let result = conn.query_drop(query)?;

    Ok(1)
}

