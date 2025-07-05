use rusqlite::{Connection, Result};

pub fn init_db() -> Result<()> {
    let conn = Connection::open("cyberlog.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL,
            role TEXT NOT NULL CHECK(role IN ('red', 'blue')),
            actions TEXT,
            results TEXT,
            vulnerabilities TEXT,
            discoveries TEXT
        );",
        [],
    )?;
    Ok(())
}

pub fn get_connection() -> Result<Connection> {
    Connection::open("cyberlog.db")
}
