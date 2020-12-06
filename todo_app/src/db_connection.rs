use rusqlite::{Connection, NO_PARAMS, params, Error};

fn get_db_connection() -> Connection {
    let conn = Connection::open("todo.db");
    if conn.is_err() {
        panic!("no db connection!");
    }
    return conn.unwrap();
}

pub fn recreate_db() {
    let conn = get_db_connection();
    let drop = conn.execute("drop table if exists todos;", NO_PARAMS);
    if drop.is_err() {
        panic!("no drop!");
    }
    let create = conn.execute(
        "create table todos (
             id integer primary key,
             name text not null,
             done integer not null
         )",
        NO_PARAMS,
    );
    if create.is_err() {
        panic!("no creation! {}", create.err().unwrap());
    }
}

pub fn insert_todo(name: String) {
    let db = get_db_connection();
    let insert = db.execute("INSERT INTO todos(name, done) values(?1,0)", params![name]);
    if insert.is_err() {
        panic!("insert error {}", insert.err().unwrap());
    }
}

struct Todo {
    id: u8,
    name: String,
    done: u8,
}

pub fn get_open_todos() -> Result<(), Error> {
    let db = get_db_connection();
    let mut select = db.prepare("select id, name, done from todos where done = 0")?;
    let todos = select.query_map(NO_PARAMS, |row| {
        Ok(Todo {
            id: row.get(0)?,
            name: row.get(1)?,
            done: row.get(2)?,
        })
    })?;
    for todo in todos {
        let ok = todo.ok().expect("");
        println!("{}: {}", ok.id, ok.name);
    }
    Ok(())
}
