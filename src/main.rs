use std::collections::hash_map::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

enum ReaderState {
    Connection,
    Request,
}

type ConnectionList = Vec<(String, String)>;

fn parse_line(s: String) -> Option<(String, String)> {
    // Line length will never be 0, as it will always have `\n` in the end.
    // If it's >1, we have some content.
    if s.len() < 2 {
        None
    } else {
        let parts: Vec<&str> = s.trim().split(" ").collect();
        let lval = parts[0].to_owned();
        let rval = parts[1].to_owned();

        Some((lval, rval))
    }
}

fn is_connected<'a>(
    conn_list: &'a ConnectionList,
    src_city: &'a String,
    tgt_city: &'a String,
    visit_map: &mut HashMap<&'a String, ()>,
) -> bool {
    // Create a list of cities connected to our `src` city.
    let conns = conn_list
        .iter()
        .filter(|(rval, lval)| rval == src_city || lval == src_city)
        .map(|(rval, lval)| if rval == src_city { lval } else { rval })
        .collect::<Vec<&String>>();

    for conn in conns.iter() {
        // Skip visited cities to not end up with infinite recursion.
        if visit_map.get(conn).is_some() {
            continue;
        }

        visit_map.insert(conn, ());

        // If we haven't visited this city already, try to find connections
        // from there.
        if *conn == tgt_city || is_connected(conn_list, conn, tgt_city, visit_map) {
            return true;
        }
    }

    false
}

fn main() -> Result<(), std::io::Error> {
    let file = File::open("cities.txt")?;
    let mut reader = BufReader::new(file);
    let mut state = ReaderState::Connection;
    let mut conn_list: ConnectionList = vec![];

    loop {
        let mut line = String::new();

        if reader.read_line(&mut line)? == 0 {
            break;
        }

        match state {
            ReaderState::Connection => match parse_line(line) {
                Some((lval, rval)) => {
                    println!("connection: lval: {} rval: {}", lval, rval);

                    conn_list.push((lval, rval));
                }

                None => state = ReaderState::Request,
            },

            ReaderState::Request => match parse_line(line) {
                Some((lval, rval)) => {
                    let status = if is_connected(&conn_list, &lval, &rval, &mut HashMap::new()) {
                        "connected"
                    } else {
                        "not connected"
                    };

                    println!("request: lval: {} rval: {}: {}", lval, rval, status);
                }

                None => continue,
            },
        }
    }

    Ok(())
}
