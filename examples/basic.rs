use crossdb::{Conn, version};

fn main() {
    println!("CrossDB version: {}", version());

    let conn = Conn::open("test.db").expect("Failed to open database");
    let result = conn.exec("SELECT * FROM system.databases;").expect("Failed to execute query");

    // Print column information
    let col_count = result.column_count();
    for i in 0..col_count {
        println!("Column {}: Type not available in current bindings", i);
    }

    // Fetch and print rows
    while let Some(row) = result.fetch_row() {
        println!("Row: {:?}", row);
    }
}
