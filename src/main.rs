use std::fs::{ self, create_dir_all, metadata, File, OpenOptions};
use std::path::{Path, PathBuf};
use chrono::Local;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};
use comfy_table::*;
use csv::{ Reader, WriterBuilder };
use std::{ env, process, error::Error, io };
use std::io::{ BufRead, BufReader, Write };
use serde::{Serialize, Deserialize};
use rand::{distributions::Alphanumeric, Rng};

struct Todo{
    id: String,
    title: String,
    description: String,
    date: String,
    completed: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct Row {
    id: String,
    title: String,
    description: String,
    date: String,
    completed: String
}

impl Todo {
    fn new(id: String, title: String, description: String) -> Self {
        Todo { id, title, description, date: Local::now().format("%Y-%m-%d %H:%M").to_string(), completed: false }
    }

    fn from_csv_string(record: &str) -> Result<Todo, Box<dyn Error>> {
        let mut inside_quotes = false;
        let mut field_start = 0;
        let mut fields = Vec::new();
        
        for (i, c) in record.chars().enumerate() {
            match c {
                '"' => inside_quotes = !inside_quotes,
                ',' if !inside_quotes => {
                    fields.push(record[field_start..i].replace("\"\"", "\"").trim_matches('"').to_string());
                    field_start = i + 1;
                },
                _ => {}
            }
        }
        // Push the last field
        fields.push(record[field_start..].replace("\"\"", "\"").trim_matches('"').to_string());
    
        if fields.len() < 5 {
            return Err("Invalid record format".into());
        }
    
        let id = fields[0].clone();
        let title = fields[1].clone();
        let description = fields[2].clone();
        let date = fields[3].clone();
        let completed = fields[4].trim() == "Yes";
    
        Ok(Todo {
            id,
            title,
            description,
            date,
            completed,
        })
    }

    fn from_csv_string_filter(record: &str, word: &str) -> Option<Todo> {
        let fields: Vec<&str> = record.split(',').collect();
    
        if fields.len() < 5 {
            return None;
        }
    
        if fields[1].to_lowercase().trim().contains(word) {
            let id = fields[0].to_string();
            let title = fields[1].to_string();
            let description = fields[2].to_string();
            let date = fields[3].to_string();
            let completed = fields[4].trim().eq_ignore_ascii_case("yes");
    
            Some(Todo {
                id,
                title,
                description,
                date,
                completed,
            })
        } else {
            None
        }
    }

}

struct Csvhelper{
    file: File,
    path: PathBuf,
}

impl Csvhelper{
    fn new(filepath: &Path) -> Result<Self, Box<dyn Error>> {
        let file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(filepath)?;

        if Csvhelper::is_file_empty(filepath.to_str().unwrap()).unwrap(){
            Csvhelper::write_headers(filepath.to_path_buf()).unwrap();
        }

        Ok(Csvhelper { file, path: filepath.to_path_buf() })
    }

    
    fn get_file_records(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut reader = Reader::from_reader(&self.file);
        let mut records = Vec::new();
    
        for result in reader.records() {
            let record = result?;
            let quoted_record: Vec<String> = record.iter()
                .map(|field| format!("\"{}\"", field.replace("\"", "\"\""))) // Quote fields and escape internal quotes
                .collect();
            records.push(quoted_record.join(","));
        }
    
        Ok(records)
    }
    

    fn print_todos(&self) {

        let todos = &self.get_file_records();

        match todos {
            Ok(csvrecords) => {

                let mut table = Table::new();
                table
                    .load_preset(UTF8_FULL)
                    .apply_modifier(UTF8_ROUND_CORNERS)
                    .set_content_arrangement(ContentArrangement::Dynamic)
                    .set_width(70)
                    .set_header(vec!["Id", "Title", "Description", "Date", "Completed"]);
                
                let mut finaltodos = Vec::new();

                for record in csvrecords {
                    match Todo::from_csv_string(&record) {
                        Ok(todo) => finaltodos.push(todo),
                        Err(e) => println!("{}", e)
                    }
                }

                if finaltodos.is_empty() {
                    println!("No se encontro ningun todo.");
                    return;
                }

                for todo in finaltodos {
                    table.add_row(vec![
                        Cell::new(todo.id.to_string()),
                        Cell::new(todo.title),
                        Cell::new(todo.description),
                        Cell::new(todo.date),
                        if todo.completed
                        {Cell::new("Yes".to_string()).fg(Color::Green)}
                        else {Cell::new("No".to_string()).fg(Color::Red)}
                    ]);
                }


                println!("{}", table);

            }

            Err(e) => {println!("{}", e);}
        }   
    }

    fn print_todos_by_condition(&self, condition: &str) {

        let todos = &self.get_file_records();

        match todos {
            Ok(csvrecords) => {

                let mut table = Table::new();
                table
                    .load_preset(UTF8_FULL)
                    .apply_modifier(UTF8_ROUND_CORNERS)
                    .set_content_arrangement(ContentArrangement::Dynamic)
                    .set_width(70)
                    .set_header(vec!["Id", "Title", "Description", "Date", "Completed"]);
                
                let mut finaltodos = Vec::new();

                for record in csvrecords {

                    if record.contains(condition) {
                        match Todo::from_csv_string(&record) {
                            Ok(todo) => finaltodos.push(todo),
                            Err(e) => println!("{}", e)
                        }
                    }

                }

                if finaltodos.is_empty() {
                    println!("No se encontro ningun todo.");
                    return;
                }

                for todo in finaltodos {
                    table.add_row(vec![
                        Cell::new(todo.id.to_string()),
                        Cell::new(todo.title),
                        Cell::new(todo.description),
                        Cell::new(todo.date),
                        if todo.completed
                        {Cell::new("Yes".to_string()).fg(Color::Green)}
                        else {Cell::new("No".to_string()).fg(Color::Red)}
                    ]);
                }


                println!("{}", table);

            }

            Err(e) => {println!("{}", e);}
        }   
    }

    fn write_headers(path: PathBuf) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)?;

        let mut wtr = WriterBuilder::new()
            .quote_style(csv::QuoteStyle::Never)
            .terminator(csv::Terminator::CRLF)
            .flexible(true)
            .delimiter(b',')
            .has_headers(false)
            .from_writer(&mut file);
    
        wtr.write_record(&["id", "title", "description", "date", "completed,"])?;
        wtr.flush()?;
        Ok(())
    }

    fn is_file_empty(file_path: &str) -> Result<bool, Box<dyn Error>> {
        let metadata = metadata(file_path)?;
        Ok(metadata.len() == 0)
    }

    fn write_todo(&self, todo: &Todo) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.path)?;
    
        let record = format!(
            "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",",
            todo.id,
            todo.title,
            todo.description,
            todo.date,
            if todo.completed { "Yes" } else { "No" }
        );
    
        // Write the record to the file directly, including the comma at the end
        writeln!(file, "{}", record)?;
    
        Ok(())
    }

    fn remove_lines_with_string(&self, target: &str) -> io::Result<()> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
    
        let original_lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
    
        let filtered_lines: Vec<String> = original_lines
            .iter()
            .filter(|line| !line.contains(target))
            .cloned()
            .collect();
    
        if original_lines.len() == filtered_lines.len() {
            println!("El todo con la ID proporcionada no existe!");
            return Ok(());
        }
    
        let mut file = OpenOptions::new().write(true).truncate(true).open(&self.path)?;
    
        for line in filtered_lines {
            writeln!(file, "{}", line)?;
        }
    
        println!("Borrado con exito!");
        Ok(())
    }

    fn generate_random_id() -> String {
        let mut rng = rand::thread_rng();
        let random_id: String = (0..4)
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect();
        random_id
    }

    fn find_todo(&self, title: &str) {
        
        let todos = &self.get_file_records();

        match todos {
            Ok(csvrecords) => {
                
                let mut table = Table::new();
                table
                    .load_preset(UTF8_FULL)
                    .apply_modifier(UTF8_ROUND_CORNERS)
                    .set_content_arrangement(ContentArrangement::Dynamic)
                    .set_width(70)
                    .set_header(vec!["Id", "Title", "Description", "Date", "Completed"]);
                
                let mut finaltodos = Vec::new();

                for record in csvrecords {
                    match Todo::from_csv_string_filter(&record, title) {
                        Some(todo) => finaltodos.push(todo),
                        None => {}
                        
                    }
                }

                if finaltodos.is_empty() {
                    println!("No se pudo encontrar coincidencias.");
                    return;
                }

                for todo in finaltodos {
                    table.add_row(vec![
                        Cell::new(todo.id.to_string()),
                        Cell::new(todo.title),
                        Cell::new(todo.description),
                        Cell::new(todo.date),
                        if todo.completed
                        {Cell::new("Yes".to_string()).fg(Color::Green)}
                        else {Cell::new("No".to_string()).fg(Color::Red)}
                    ]);
                }


                println!("{}", table);

            },

            Err(_) => println!("No se pudo escanear y encontrar coincidencias!")
        }

    }

    fn mark_todo(&self, id: &str) -> io::Result<()> {
        let file = File::open(&self.path)?;
        let reader: BufReader<File> = BufReader::new(file);

        let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

        let mut file = OpenOptions::new().write(true).truncate(true).open(&self.path)?;
        let mut change: bool = false;

        for line in lines {
            if line.contains(id) {
                if change == false {change = true;}
                if line.contains("No")
                    {writeln!(file, "{}", line.replace("No", "Yes"))?; println!("Marcado correctamente a Si!")}
                else
                    {writeln!(file, "{}", line.replace("Yes", "No"))?; println!("Marcado correctamente a No!")}
            } else {
                writeln!(file, "{}", line)?;
            }            
        }

        if !change {println!("No se ha encontrado coincidencias!");}

        Ok(())
    }

    fn delete_all_todos(&self) -> io::Result<()> {
        let file = fs::read_to_string(&self.path)?;

        let mut lines = file.lines();

        if let Some(headers) = lines.next() {
            let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)?;

            writeln!(file, "{}", headers).unwrap();
            println!("Tareas borradas correctamente!");

            Ok(())
        } else {
            println!("No se pudo realizar la operacion, el archivo esta vacio o no existe!");
            Ok(())
        }
    }

}

fn main() {
    let appdata = env::var("LOCALAPPDATA").unwrap_or_else(|_| String::from(""));
    let mut app_dir = PathBuf::from(appdata);
    app_dir.push("TodoCLI");

    if !app_dir.exists() {
        create_dir_all(&app_dir).unwrap();
    }

    let filepath = app_dir.join("data.csv");
    let csv_assistant = match Csvhelper::new(&filepath) {
        Ok(csv) => {
            csv
        }
        Err(e) => {
            println!("Cannot create database of todos: {}", e);
            process::exit(1);
        }
    };

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Uso: todo <command>");
        println!("Commands:");
        println!("  print        Imprimir los todos");
        println!("  print-yes    Imprimir los todos completados");
        println!("  print-no     Imprimir los todos no completados");
        println!("  add          Añadir un todo");
        println!("  find         Encontrar un todo por titulo");
        println!("  mark         Marcar un todo como completado o incompleto");
        println!("  delete       Borrar un todo por ID");
        println!("  delete-all   Borrar un todo por ID");
        return;
    }
    
    match args[1].as_str() {
        "print" => csv_assistant.print_todos(),
        
        "add" => {
            loop {

                println!("Title: ");

                let mut tit = String::new();
                let mut des = String::new();

                io::stdin()
                    .read_line(&mut tit)
                    .expect("Please put a correct input!\n");

                println!("Description: ");

                io::stdin()
                    .read_line(&mut des)
                    .expect("Please put a correct input!\n");

                let new: Todo = Todo::new(
                    Csvhelper::generate_random_id(),
                    tit.trim().to_string(),
                    des.trim().to_string()
                );

                csv_assistant.write_todo(&new).unwrap();
                break;

            }
        },

        "delete" => {

            loop {

                println!("Introduce el ID del todo: ");
                let mut word: String = String::new();
                io::stdin()
                    .read_line(&mut word).expect("No se pudo leer el indice!");

                match csv_assistant.remove_lines_with_string(&word.as_str().trim()) {
                    Ok(()) => {},
                    Err(_) => println!("No se pudo eliminar, no existe!")
                }

                break;

            }

        },

        "find" => {

            loop {
                
                println!("Cual es el posible titulo: ");
                let mut possible_title: String = String::new();
                io::stdin()
                    .read_line(&mut possible_title).expect("No se pudo entender la palabra dada!\n");

                let possible_title = possible_title.trim().to_ascii_lowercase();

                csv_assistant.find_todo(&possible_title);

                break;

            }

        },

        "mark" => {

            loop {
                
                println!("Cual es la ID del todo a marcar: ");
                let mut possible_id: String = String::new();
                io::stdin()
                    .read_line(&mut possible_id).expect("No se pudo entender la palabra dada!\n");

                let possible_id:&str = possible_id.trim();

                match csv_assistant.mark_todo(possible_id) {
                    Ok(()) => {},
                    Err(_) => println!("No se pudo remplazar nada, probablemente archivo vacio!")
                }

                break;

            }

        },

        "print-yes" => csv_assistant.print_todos_by_condition("Yes"),

        "print-no" => csv_assistant.print_todos_by_condition("No"),

        "delete-all" => {
            loop {
                
                println!("Quieres borrrar todas las tareas?: (s/n)");
                let mut user_input: String = String::new();
                io::stdin()
                    .read_line(&mut user_input).expect("No entendi bien, puedes ser mas claro?");

                match user_input.trim().to_lowercase().as_str() {
                    "s" => {
                        csv_assistant.delete_all_todos().unwrap();
                        println!("Borrados correctamente.");
                    },

                    _ => println!("No se han borrado")
                }

                break;

            }
        }

        _ => {
            
            println!("Uso: todo <command>");
            println!("Commands:");
            println!("  print        Imprimir los todos");
            println!("  print-yes    Imprimir los todos completados");
            println!("  print-no     Imprimir los todos no completados");
            println!("  add          Añadir un todo");
            println!("  find         Encontrar un todo por titulo");
            println!("  mark         Marcar un todo como completado o incompleto");
            println!("  delete       Borrar un todo por ID");
            println!("  delete-all   Borrar un todo por ID");

        }
    }

}
