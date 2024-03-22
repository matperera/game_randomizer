use lazy_static::lazy_static;
use rusqlite::{Connection, Result};
use std::sync::Mutex;

// Struct for the commands table
// #[allow(dead_code)] is used to suppress the warning for the struct being unused
// #[derive(Debug)] is used to derive the Debug trait for the struct
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Console {
    pub id: Option<i32>,
    pub console: String,
    pub plays_left: i32,
    pub is_handheld: i32,
}

//Hard coded for my use
lazy_static! {
    static ref CONSOLES_VEC: Mutex<Vec<Console>> = Mutex::new(vec![
        Console {
            id: None,
            console: "SteamDeck".to_string(),
            plays_left: 5,
            is_handheld: 1,
        },
        Console {
            id: None,
            console: "Switch".to_string(),
            plays_left: 5,
            is_handheld: 1,
        },
        Console {
            id: None,
            console: "PS5".to_string(),
            plays_left: 5,
            is_handheld: 0,
        },
        Console {
            id: None,
            console: "PC".to_string(),
            plays_left: 5,
            is_handheld: 0,
        },
        Console {
            id: None,
            console: "DS".to_string(),
            plays_left: 5,
            is_handheld: 1,
        },
        Console {
            id: None,
            console: "3DS".to_string(),
            plays_left: 5,
            is_handheld: 1,
        },
        Console {
            id: None,
            console: "GameBoy".to_string(),
            plays_left: 5,
            is_handheld: 1,
        },
        Console {
            id: None,
            console: "Wii".to_string(),
            plays_left: 5,
            is_handheld: 0,
        },
        Console {
            id: None,
            console: "GameCube".to_string(),
            plays_left: 5,
            is_handheld: 0,
        },
        Console {
            id: None,
            console: "PS2".to_string(),
            plays_left: 5,
            is_handheld: 0,
        },
    ]);
}

pub fn setup() -> Result<()> {
    let data = check_table()?;

    if data {
        return Ok(());
    }

    let conn = Connection::open("./.consoles.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS console_plays (
            id INTEGER PRIMARY KEY,
            console TEXT NOT NULL UNIQUE,
            plays_left INTEGER NOT NULL,
            is_handheld INTEGER DEFAULT 0
        )",
        [],
    )?;

    let consoles = CONSOLES_VEC.lock().unwrap();
    for console in consoles.iter() {
        conn.execute(
            "INSERT INTO console_plays (console, plays_left, is_handheld) VALUES (?, ?, ?)",
            [&console.console, &console.plays_left.to_string(), &console.is_handheld.to_string()],
        )?;
    }

    Ok(())
}

#[allow(private_interfaces)]
pub fn get_consoles(is_handheld: Option<i32>) -> Result<Vec<Console>> {
    let conn = Connection::open("./.consoles.db")?;

    let sql_query = match is_handheld {
        Some(1) => "SELECT * FROM console_plays WHERE plays_left > 0 AND is_handheld = 1",
        Some(0) => "SELECT * FROM console_plays WHERE plays_left > 0 AND is_handheld = 0",
        _ => "SELECT * FROM console_plays WHERE plays_left > 0",
    };
    let mut stmt = conn.prepare(sql_query)?;
    let plays_iter = stmt.query_map([], |row| {
        Ok(Console {
            id: row.get(0)?,
            console: row.get(1)?,
            plays_left: row.get(2)?,
            is_handheld: row.get(3)?,
        })
    })?;

    let mut consoles = Vec::new();

    for play in plays_iter {
        consoles.push(play?);
    }

    Ok(consoles)
}

pub fn find_one_console(console: &str) -> Result<Console> {
    let conn = Connection::open("./.consoles.db")?;

    let mut stmt = conn.prepare("SELECT * FROM console_plays WHERE console LIKE ? LIMIT 1")?;
    let plays_iter = stmt.query_map([console], |row| {
        Ok(Console {
            id: row.get(0)?,
            console: row.get(1)?,
            plays_left: row.get(2)?,
            is_handheld: row.get(3)?,
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
    let conn = Connection::open("./.consoles.db")?;

    conn.execute("UPDATE console_plays SET plays_left = 5", [])?;
    Ok(())
}

pub fn update_one_counter(console: &str) -> Result<()> {
    let conn = Connection::open("./.consoles.db")?;

    conn.execute("UPDATE console_plays SET plays_left = plays_left - 1 WHERE console = ?", [console])?;
    Ok(())
}

pub fn update_console_counter(console: &str, new_count: &str) -> Result<()> {
    let conn = Connection::open("./.consoles.db")?;

    conn.execute("UPDATE console_plays SET plays_left = ?1 WHERE console LIKE ?2", [new_count, console])?;
    Ok(())
}

pub fn check_table() -> Result<bool, rusqlite::Error> {
    let conn = Connection::open("./.consoles.db")?;

    let table_exists = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='console_plays'")?
        .exists([])?;

    
    Ok(table_exists)
}
