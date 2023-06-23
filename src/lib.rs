use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use log::{Level, info};

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Debug, Eq, PartialEq, Clone)]
struct Example {
    pub id: i64,
    pub name: String,
}

#[wasm_bindgen_test]
fn test_rusqlite() {
    console_log::init_with_level(Level::Debug);

    let conn = rusqlite::Connection::open_in_memory().unwrap();

    conn.execute(
        "CREATE TABLE example (id INT, name TEXT);",
        [], // empty list of parameters.
    ).unwrap();

    let val = Example {
        id: 1,
        name: "test".into(),
    };
    conn.execute(
        "INSERT INTO example (id, name) VALUES (?, ?)",
        (val.id, val.name.clone()),
    )
    .expect("Failed to set");
    let res = conn
        .query_row(
            "SELECT id, name FROM example WHERE (id=?)",
            [val.id],
            |row| {
                Ok(Example {
                    id: row.get(0).unwrap(),
                    name: row.get(1).unwrap(),
                })
            },
        )
        .unwrap();
    info!("{:?}", res);
    assert_eq!(res, val);
    panic!();
}
