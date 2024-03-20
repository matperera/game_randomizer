use rusqlite::{Connection, Result};

// Struct for the commands table
// #[allow(dead_code)] is used to suppress the warning for the struct being unused
// #[derive(Debug)] is used to derive the Debug trait for the struct
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Console {
    pub id: Option<i32>,
    pub console: String,
    pub plays_left: i32,
}

pub fn setup() -> Result<()> {
    let data = check_table()?;

    if data {
        return Ok(());
    }

    let conn = Connection::open("./consoles.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS console_plays (
            id INTEGER PRIMARY KEY,
            console TEXT NOT NULL UNIQUE,
            plays_left INTEGER NOT NULL
        )",
        [],
    )?;

   let consoles = vec![
        "SteamDeck",
        "Switch",
        "PS5",
        "PC",
        "DS",
        "3DS",
        "GameBoy",
        "Wii",
        "GameCube",
        "PS2"
    ];

    for console in consoles {
        conn.execute(
            "INSERT INTO console_plays (console, plays_left) VALUES (?, ?)",
            [console, "5"],
        )?;
    }

    Ok(())
}

#[allow(private_interfaces)]
pub fn get_consoles() -> Result<Vec<Console>> {
    let conn = Connection::open("./consoles.db")?;

    let mut stmt = conn.prepare("SELECT * FROM console_plays WHERE plays_left > 0")?;
    let plays_iter = stmt.query_map([], |row| {
        Ok(Console {
            id: row.get(0)?,
            console: row.get(1)?,
            plays_left: row.get(2)?,
        })
    })?;

    let mut consoles = Vec::new();

    for play in plays_iter {
        consoles.push(play?);
    }

    Ok(consoles)
}

pub fn find_one_console(console: &str) -> Result<Console> {
    let conn = Connection::open("./consoles.db")?;

    let mut stmt = conn.prepare("SELECT * FROM console_plays WHERE console LIKE ? LIMIT 1")?;
    let plays_iter = stmt.query_map([console], |row| {
        Ok(Console {
            id: row.get(0)?,
            console: row.get(1)?,
            plays_left: row.get(2)?,
        })
    })?;

    let mut consoles = Vec::new();

    for play in plays_iter {
        consoles.push(play?);
    }

    if consoles.is_empty() {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    Ok(consoles[0].clone())
}

pub fn reset_counters() -> Result<()> {
    let conn = Connection::open("./consoles.db")?;

    conn.execute("UPDATE console_plays SET plays_left = 5", [])?;
    Ok(())
}

pub fn update_one_counter(console: &str) -> Result<()> {
    let conn = Connection::open("./consoles.db")?;

    conn.execute("UPDATE console_plays SET plays_left = plays_left - 1 WHERE console = ?", [console])?;
    Ok(())
}

pub fn update_console_counter(console: &str, new_count: &str) -> Result<()> {
    let conn = Connection::open("./consoles.db")?;

    conn.execute("UPDATE console_plays SET plays_left = ?1 WHERE console LIKE ?2", [new_count, console])?;
    Ok(())
}

pub fn check_table() -> Result<bool, rusqlite::Error> {
    let conn = Connection::open("./consoles.db")?;

    let table_exists = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='console_plays'")?
        .exists([])?;

    
    Ok(table_exists)
}
