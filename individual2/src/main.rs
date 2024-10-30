use rand::Rng;
use rusqlite::{params, Connection, Result};
use std::io::{self, Write};

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
}

fn create_user(conn: &Connection, name: &str) -> Result<usize> {
    conn.execute("INSERT INTO user (name) VALUES (?1)", params![name])
}

fn read_user(conn: &Connection, id: i32) -> Result<User> {
    let mut stmt = conn.prepare("SELECT id, name FROM user WHERE id = ?1")?;
    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}

fn update_user(conn: &Connection, id: i32, name: &str) -> Result<usize> {
    conn.execute("UPDATE user SET name = ?1 WHERE id = ?2", params![name, id])
}

fn delete_user(conn: &Connection, id: i32) -> Result<usize> {
    conn.execute("DELETE FROM user WHERE id = ?1", params![id])
}

fn main() -> Result<()> {
    let conn = Connection::open("users.db")?;

    // Create
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
            id      INTEGER PRIMARY KEY,
            name    TEXT NOT NULL
        )",
        [],
    )?;

    // Prompt for ID and name
    let mut id_input = String::new();
    print!("Enter user ID: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut id_input).unwrap();
    let id: i32 = match id_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid ID input.");
            return Ok(());
        }
    };

    let mut name = String::new();
    print!("Enter user name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    // Randomly select a CRUD operation
    let mut rng = rand::thread_rng();
    let operation = rng.gen_range(0..4);

    match operation {
        0 => {
            // Create
            println!("Operation: Create");
            match create_user(&conn, name) {
                Ok(_) => println!("User '{}' created successfully.", name),
                Err(e) => eprintln!("Error creating user: {}", e),
            }
        }
        1 => {
            // Read
            println!("Operation: Read");
            match read_user(&conn, id) {
                Ok(user) => println!("User found: ID: {}, Name: {}", user.id, user.name),
                Err(e) => eprintln!("Error reading user: {}", e),
            }
        }
        2 => {
            // Update
            println!("Operation: Update");
            match update_user(&conn, id, name) {
                Ok(rows_updated) => {
                    if rows_updated > 0 {
                        println!("User ID {} updated to '{}'.", id, name);
                    } else {
                        println!("No user found with ID {}.", id);
                    }
                }
                Err(e) => eprintln!("Error updating user: {}", e),
            }
        }
        3 => {
            // Delete
            println!("Operation: Delete");
            match delete_user(&conn, id) {
                Ok(rows_deleted) => {
                    if rows_deleted > 0 {
                        println!("User ID {} deleted successfully.", id);
                    } else {
                        println!("No user found with ID {}.", id);
                    }
                }
                Err(e) => eprintln!("Error deleting user: {}", e),
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
