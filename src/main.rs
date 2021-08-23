use std::collections::HashMap;

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().expect("Key was not there");
    let value = args.next().unwrap();
    println!("the key is '{}' and the value is '{}'", key, value);
    
   
    let mut database = Database::new().expect("Creating db failed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    database.flush().unwrap();
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // read the rt.db
        //let contents = match std::fs::read_to_string("rt.db") {
        //    Ok(c) => c,
        //    Err(error) => {
        //        return Err(error);
        //   }
        // };
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("rt.db")?;
        for line in contents.lines() {
            let mut chunks = line.splitn(2,'\t');
            let key = chunks.next().expect("no key!");
            let value = chunks.next().expect("no val you!");
            map.insert(key.to_owned(), value.to_owned());
    
        }
        // parse the string

        // popul8 map
        Ok(Database { map })
        
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(self) -> std::io::Result<()> {
        let mut contents = String::new();
        for (key, value)in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }
    std::fs::write("rt.db", contents)
    }

}
 