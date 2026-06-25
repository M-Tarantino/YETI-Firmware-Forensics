use rusqlite::{Connection, params};
use crate::util::error::YetiResult;
use crate::core::scanner::Signature;

/// The central repository for forensic signatures and vendor-specific DNA.
pub struct KnowledgeBase {
    conn: Connection,
}

impl KnowledgeBase {
    pub fn new(path: &str) -> YetiResult<Self> {
        let conn = Connection::open(path)?;
        let kb = Self { conn };
        kb.initialize_schema()?;
        kb.migrate_signatures()?;
        Ok(kb)
    }

    fn initialize_schema(&self) -> YetiResult<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS signatures (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                magic TEXT NOT NULL,
                category TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    fn migrate_signatures(&self) -> YetiResult<()> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM signatures",
            [],
            |r| r.get(0)
        )?;

        if count == 0 {
            log::info!("KnowledgeBase is empty. Bootstrapping forensic DNA...");
            let default_sigs = vec![
                ("SquashFS_be", "73717368", "Filesystem"),
                ("SquashFS_le", "68737173", "Filesystem"),
                ("LZMA_Stream", "5d000080", "Compression"),
                ("ELF_Binary",  "7f454c46", "Executable"),
            ];

            for (name, magic, cat) in default_sigs {
                self.conn.execute(
                    "INSERT INTO signatures (name, magic, category) VALUES (?, ?, ?)",
                    params![name, magic, cat],
                )?;
            }
        }
        Ok(())
    }

    /// Loads signatures from the database, optionally filtered by a specific category.
    pub fn load_signatures(&self, category: Option<&str>) -> YetiResult<Vec<Signature>> {
        // Fix: We execute the query inside each match arm to ensure 'c' lives long enough.
        match category {
            Some(c) => {
                let mut stmt = self.conn.prepare(
                    "SELECT name, magic, category FROM signatures WHERE category = ?"
                )?;
                let rows = stmt.query_map(params![c], |row| self.map_row(row))?;
                self.collect_results(rows)
            }
            None => {
                let mut stmt = self.conn.prepare(
                    "SELECT name, magic, category FROM signatures"
                )?;
                let rows = stmt.query_map([], |row| self.map_row(row))?;
                self.collect_results(rows)
            }
        }
    }

    /// Helper to map a database row to a Signature struct
    fn map_row(&self, row: &rusqlite::Row) -> rusqlite::Result<Signature> {
        let name: String = row.get(0)?;
        let magic_hex: String = row.get(1)?;
        let category: String = row.get(2)?;

        Ok(Signature {
            name,
            magic: hex::decode(magic_hex).unwrap_or_else(|_| vec![]),
            category,
        })
    }

    /// Helper to collect the mapped rows into a Vector
    fn collect_results(&self, rows: rusqlite::MappedRows<impl FnMut(&rusqlite::Row) -> rusqlite::Result<Signature>>) -> YetiResult<Vec<Signature>> {
        let mut results = Vec::new();
        for row_result in rows {
            results.push(row_result?);
        }
        Ok(results)
    }
}