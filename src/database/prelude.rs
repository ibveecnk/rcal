fn connect_to_sqlite() -> rusqlite::Connection {
    rusqlite::Connection::open_in_memory().unwrap()
}
