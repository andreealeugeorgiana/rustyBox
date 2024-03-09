/* Suggestions: 1. Write a function for every command
                2. Start with the pwd command
                3. Continue with the other commands that do not have parameters
*/

//cargo install --path .. ;./run_all.sh

use std::{fs, os::unix::fs::symlink, io::{Read, Write}};
use std::path::Path;
use std::os::unix::fs::PermissionsExt;


fn pwd() -> Result<(),()> {
    // TODO 3: Implement the logic for pwd
    let path = std::env::current_dir();
    match path {
        Ok(path) => if let Some(dir) = path.to_str() {
            return Ok(println!("{}", dir));
        } else {
            return Err(println!("Could not find path!"));
        }
        Err(_path) => return Err(println!("Could not find path!")),
    }

}

fn echo(message: &String) -> Result<(), i32>{
    if message.len() < 2 {
        return Err(-10);
    } else {
        let output = message.trim();
        return Ok(print!("{}", output));
    }
}

fn cat(files :&[String]) -> Result<(), i32> {
    match files.len() {
        0 => return Err(-20),
        _ => {
            for file in files {
                match fs::metadata(file) {
                    Ok(_) => {
                        let contents = fs::read_to_string(file);
                        match contents {
                            Ok(contents) => print!("{}", contents),
                            Err(_) => return Err(-20),
                        }
                    },
                    Err(_) => return Err(-20),
                }
            }
            return Ok(())
        }
    }
}

fn mkdir(files: &[String]) -> Result<(), i32> {
    match files.len() {
        0 => return Err(-30),
        _ => {
            for file in files {
                match fs::metadata(file) {
                    Ok(_) => return Ok(()),
                    Err(_) => {
                        match fs::File::create(file){
                            Ok(_) => return Ok(()),
                            Err(_) => return Err(-30),
                        }
                    }
                }
            }
            return Ok(())
        }
    }
}

fn mv(name: &String, rename: &String) -> Result<(), i32> {
    match fs::rename(name, rename) {
        Ok(_) => return Ok(()),
        Err(_) => return Err(-40),
    }
}

fn ln(args: &[String]) -> Result<(), i32> {
    if args.len() < 2 {
        return Err(-50);
    } else if args.len() == 3 && (args[0] == "-s" || args[0] == "--symbolic") {
        match symlink(&args[1], &args[2]) {
            Ok(_) => return Ok(()),
            Err(_) => return Err(-50),
        }
    } else if args.len() == 2 {
        match fs::hard_link(&args[0], &args[1]) {
            Ok(_) => return Ok(()),
            Err(_) => return Err(-50),
        }
    } else {
        return Err(-50)
    }
}

fn rmdir(directories: &[String]) -> Result<(), i32> {
    
    for directory in directories {
        let contents = fs::read_dir(directory);
        match contents {
            Ok(contents) => {
                for _ in contents {
                    return Err(-60);
                }
                match fs::remove_dir(directory) {
                    Ok(_) => (),
                    Err(_) => (),
                }
            },
            Err(_) => return Err(-60),
        }
    }
    return Ok(());
}

fn rm(args: &[String]) -> Result<i32, i32> {
    if args[0] == "-r" || args[0] == "-R" || args[0] == "--recursive" {
        let to_del = &args[1..];
        let mut is_error = false;
        for hope_dir in to_del {
            match fs::read_dir(hope_dir) {
                Ok(_result) => {
                    match fs::remove_dir_all(hope_dir) {
                        Ok(_) => (),
                        Err(_) => {
                            is_error = true;
                        },
                    }
                },
                Err(_err) => {
                    let hope_file = fs::metadata(hope_dir);
                    match hope_file {
                        Ok(_) => {
                            match fs::remove_file(hope_dir) {
                                Ok(_) => (),
                                Err(_) => {
                                    is_error = true;
                                },
                            }
                        },
                        Err(_) => (),
                    }
                },
            }
        }
        if is_error {
            return Err(-70);
        }
            return Ok(0);
    } else if args[0] == "-d" || args[0] == "--dir" {
        let to_del = &args[1..];
        match rmdir(to_del) {
            Ok(_) => return Ok(0),
            Err(_) => return Err(-70),
        }
    } else {
        let to_del = &args[0..];
        let mut is_error = false;
        for file in to_del {
            let hope_file = fs::metadata(file);
            match hope_file {
                Ok(_) => {
                    match fs::remove_file(file) {
                        Ok(_) => (),
                        Err(_) => {
                            is_error = true;
                        },
                    }
                },
                Err(_) => {
                    is_error = true;
                },
            }
        }
        if is_error {
            return Err(-70);
        }
        Ok(0)
    }
}

fn ls_recursive(path: &str, not_secret: bool) -> Result<(), i32> {
    let dir = fs::read_dir(path);
    match dir {
        Ok(dir) => {
            println!("{}:", path);
            if not_secret == true {
                println!(".");
                println!("..");
            }
            for content in dir {
                match content {
                    Ok(content) => {
                        let display = content.file_name();
                        if not_secret == false {
                            let not_secret = display.to_string_lossy();
                            if not_secret.starts_with('.') {
                                continue;
                            }
                            println!("{}", display.to_string_lossy().to_string());
                        } else {
                            println!("{}", display.to_string_lossy().to_string());
                        }
                        let path = content.path();
                        if path.is_dir() {
                            if let Err(_) = ls_recursive(&path.to_string_lossy(), not_secret) {
                                return Err(-80);
                            }
                        }
                    }
                    Err(_) => return Err(-80),
                }
            }
            Ok(())
        }
        Err(_) => return Err(-80),
    }
}

fn ls(args: &[String]) -> Result<(), i32> {
    if args.len() > 0 && args[0].starts_with('-') {
        if args[0] == "-a" || args[0] == "-all" {
            if args.len() == 1 {
                let path = std::env::current_dir();
                match path {
                    Ok(path) => {
                        let contents = fs::read_dir(&path);
                        match contents {
                        Ok(contents) => {
                            println!(".");
                            println!("..");
                            for content in contents {
                                match content {
                                    Ok(content) => {
                                        let display = content.file_name();
                                        println!("{}", display.to_string_lossy().to_string())
                                    },
                                    Err(_) => (),
                                }
                            }
                        },
                        Err(_) => return Err(-80),
                        }
                    },
                    Err(_) => return Err(-80),
                }
            } else {
                let to_show = &args[1..];
                for entry in to_show {
                    let contents = fs::metadata(&entry);
                    match contents {
                        Ok(contents) => {
                            if contents.is_dir() {
                                let dir = fs::read_dir(&entry);
                                match dir {
                                    Ok(dir) => { 
                                        println!(".");
                                        println!("..");
                                        for content in dir {
                                            match content {
                                                Ok(content) => {
                                                    let display = content.file_name();
                                                    println!("{}", display.to_string_lossy().to_string())
                                                },
                                                Err(_) => return Err(-80),
                                            }
                                        }
                                    },
                                    Err(_) => return Err(-80),
                                }
                            } else if contents.is_file() {
                                println!("{}", contents.is_file().to_string())
                            }
                        },
                        Err(_) => (),
                    }
                }
            }
        } else if args[0] == "-R" || args[0] == "--recursive" {
            if args.len() > 1 {
                if args[1] == "-a" || args[1] == "-all" { 
                    for entry in &args[2..] {
                        if let Err(_) = ls_recursive(&entry, true) {
                            return Err(-80);
                        }
                    }
                } else {
                    for entry in &args[1..] {
                        if let Err(_) = ls_recursive(&entry, false) {
                            return Err(-80);
                        }
                    }
                }
            } else {
                if let Err(_) = ls_recursive(&".", false) {
                    return Err(-80);
                }
            }
        }
        Ok(())


    } else if args.len() == 1 {
        let path = std::path::Path::new(&args[0]);

        if path.is_file() {
            println!("{}", path.display());
            return Ok(());
        } else {
            let dir = fs::read_dir(&path);
            match dir {
                Ok(dir) => {
                    for content in dir {
                        match content {
                            Ok(content) => {
                                let display = content.file_name();
                                let not_secret = display.to_string_lossy();
                                if not_secret.starts_with('.') {
                                    continue;
                                } else {
                                    println!("{}", not_secret);
                                }
                            },
                            Err(_) => return Err(-80),
                        }
                    }
                    Ok(())
                },
                Err(_) => return Err(-80),
            }
        }

    } else {
        let path = std::env::current_dir();
        match path {
            Ok(path) => {
                let contents = fs::read_dir(path);
                match contents {
                Ok(path) => {
                    for content in path {
                        match content {
                            Ok(content) => {
                                let display = content.file_name();
                                let not_secret = display.to_string_lossy();
                                if not_secret.starts_with('.') {
                                    continue;
                                } else {
                                    println!("{}", not_secret);
                                }
                            },
                            Err(_) => (),
                        }
                    }
                    Ok(())
                },
                Err(_) => return Err(-80),
                }
            },
            Err(_) => return Err(-80),
        }
    }
}

fn cp_r(source: &std::path::Path, destination: &std::path::Path) -> Result<(), i32> {
    if source.is_file(){
        if destination.is_dir() {
            let mut dest_file_path = destination.to_path_buf();
            dest_file_path.push(source.file_name().unwrap());
            match fs::copy(source, &dest_file_path) {
                Ok(_) => (),
                Err(_) => return Err(-90),
            }
        }
    } else if source.is_dir() {
        if !destination.exists() {
            match fs::create_dir_all(destination) {
                Ok(_) => (),
                Err(_) => return Err(-90),
            }
            match fs::read_dir(source) {
                Ok(entries) => {
                    for entry in entries {
                        let entry = entry.unwrap();
                        let entry_path = entry.path();
                        let new_destination = destination.join(entry_path.file_name().unwrap());
                        if entry_path.is_dir() {
                            match cp_r(&entry_path, &new_destination) {
                                Ok(_) => (),
                                Err(_) => return Err(-90),
                            }
                        } else {
                            match fs::copy(&entry_path, &new_destination) {
                                Ok(_) => (),
                                Err(_) => return Err(-90),
                            }
                        }
                    }
                },
                Err(_) => return Err(-90),
            }
        } else {
            let dest_dir_path = destination.join(source.file_name().unwrap());
            match fs::create_dir_all(&dest_dir_path) {
                Ok(_) => (),
                Err(_) => return Err(-90),
            }
            match fs::read_dir(source) {
                Ok(entries) => {
                    for entry in entries {
                        let entry = entry.unwrap();
                        let entry_path = entry.path();
                        let new_destination = dest_dir_path.join(entry_path.file_name().unwrap());
                        if entry_path.is_dir() {
                            match cp_r(&entry_path, &new_destination) {
                                Ok(_) => (),
                                Err(_) => return Err(-90),
                            }
                        } else {
                            match fs::copy(&entry_path, &new_destination) {
                                Ok(_) => (),
                                Err(_) => return Err(-90),
                            }
                        }
                    }
                },
                Err(_) => return Err(-90),
            }
        }
    }
    Ok(())
}

fn cp(source: &String, destination: &String, recursive: bool) -> Result <(), i32> {
    if recursive {
        if !destination.is_empty() {
            match cp_r(std::path::Path::new(source), std::path::Path::new(destination)) {
                Ok(_) => Ok(()),
                Err(_) => return Err(-90),
            }
        } else {
            match std::env::current_dir() {
                Ok(path) => {
                    match cp_r(std::path::Path::new(source), std::path::Path::new(&path)) {
                        Ok(_) => Ok(()),
                        Err(_) => return Err(-90),
                    }
                },
                Err(_) => return Err(-90),
            }
        }
    } else {
        let src = fs::File::open(source);
        match src {
            Ok(mut src) => {
                let dest_path = Path::new(destination);
                if dest_path.is_dir() {
                    let dest_file = dest_path.join(Path::new(source).file_name().unwrap());
                    let dest = fs::File::create(&dest_file);
                    match dest {
                        Ok(mut dest) => {
                            let mut buffer = Vec::new();
                            if let Err(_) = src.read_to_end(&mut buffer) {
                                return Err(-90);
                            }
                            if let Err(_) = dest.write_all(&buffer) {
                                return Err(-90);
                            }
                            Ok(())
                        }
                        Err(_) => return Err(-90),
                    }
                } else {
                    let dest = fs::File::create(destination);
                    match dest {
                        Ok(mut dest) => {
                            let mut buffer = Vec::new();
                            if let Err(_) = src.read_to_end(&mut buffer) {
                                return Err(-90);
                            }
                            if let Err(_) = dest.write_all(&buffer) {
                                return Err(-90);
                            }
                            Ok(())
                        }
                        Err(_) => return Err(-90),
                    }
                }
            }
            Err(_) => return Err(-90),
        }
    }
}

fn touch(args: &[String]) -> Result<(), i32> {
    if args[0] == "-a"  {
        if args[1] == "-c" || args[1] == "--no-create" {
            if Path::new(&args[2]).exists() {
                match fs::File::open(&args[2]) {
                    Ok(mut file) => {
                        let mut contents = String::new();
                        match file.read_to_string(&mut contents) {
                            Ok(_) => Ok(()),
                            Err(_) => return Err(-100),
                        }
                    },
                    Err(_) => return Err(-100),
                }
            } else {
                return Ok(());
            }
        } else {
            if Path::new(&args[1]).exists() {
                match fs::File::open(&args[1]) {
                    Ok(mut file) => {
                        let mut contents = String::new();
                        match file.read_to_string(&mut contents) {
                            Ok(_) => Ok(()),
                            Err(_) => return Err(-100),
                        }
                    },
                    Err(_) => return Err(-100),
                }
            } else {
                let file = fs::File::create(&args[1]);
                match file {
                    Ok(_) => Ok(()),
                    Err(_) => return Err(-100),
                }
            }
        }
    } else if args[0] == "-m" {
        if args[1] == "-c" || args[1] == "--no-create" {
            if Path::new(&args[2]).exists() {
                match fs::File::create(&args[2]) {
                    Ok(mut file) => {
                        let contents = 'a';
                        match file.write_all(contents.to_string().as_bytes()) {
                            Ok(_) => Ok(()),
                            Err(_) => return Err(-100),
                        }
                    },
                    Err(_) => return Err(-100),
                }
            } else {
                return Ok(());
            }
        } else {
            if Path::new(&args[1]).exists() {
                match fs::File::create(&args[1]) {
                    Ok(mut file) => {
                        let contents = " ";
                        match file.write_all(contents.as_bytes()) {
                            Ok(_) => Ok(()),
                            Err(_) => return Err(-100),
                        }
                    },
                    Err(_) => return Err(-100),
                }
            } else {
                let file = fs::File::create(&args[1]);
                match file {
                    Ok(_) => Ok(()),
                    Err(_) => return Err(-100),
                }
            }
        }
    } else if args[0] == "-c" || args[0] == "--no-create" {
        if Path::new(&args[1]).exists() {
            match fs::File::open(&args[1]) {
                Ok(_) => {
                    match fs::File::create(&args[1]) {
                        Ok(file) => {
                            match cp(&args[1], &fs::read_to_string(file.metadata().unwrap().is_file().to_string()).unwrap(), false) {
                                Ok(_) => {
                                    match fs::remove_file(&args[1]) {
                                        Ok(_) => Ok(()),
                                        Err(_) => return Err(-100),
                                    }
                                },
                                Err(_) => return Err(-100),
                            }
                        },
                        Err(_) => return Err(-100),
                    }
                },
                Err(_) => return Err(-100),
            }
        } else {
            return Ok(());
        }
    } else {
        if Path::new(&args[0]).exists() {
            match fs::File::open(&args[0]) {
                Ok(_) => {
                    match fs::remove_file(&args[0]) {
                        Ok(_) => {
                            match fs::File::create(&args[0]) {
                                Ok(mut file) => {
                                    let content = "a";
                                    match file.write_all(content.as_bytes()) {
                                        Ok(_) => Ok(()),
                                        Err(_) => return Err(-100),
                                    }
                                },
                                Err(_) => return Err(-100),
                            }
                        },
                        Err(_) => return Err(-100),
                    }
                },
                Err(_) => return Err(-100),
            }
        } else {
            let file = fs::File::create(&args[0]);
            match file {
                Ok(_) => Ok(()),
                Err(_) => return Err(-100),
            }
        }
    }
}

fn chmod(file_path: &str, mode: &str) -> Result<(), i32> {
    if mode.chars().any(|c| "+".contains(c)) {
        let mut user_bits = 0o0;
        let mut group_bits = 0o0;
        let mut other_bits = 0o0;

        let mut change = '+';
        let mut permission_bits = String::new();

        for c in mode.chars() {
            match c {
                '+' | '-' | '=' => change = c,
                'r' | 'w' | 'x' => permission_bits.push(c),
                'a' => {
                    user_bits |= 0o7;
                    group_bits |= 0o7;
                    other_bits |= 0o7;
                }
                _ => {
                    if change == '+' {
                        if c == 'u' {
                            user_bits |= 0o7;
                        } else if c == 'g' {
                            group_bits |= 0o7;
                        } else if c == 'o' {
                            other_bits |= 0o7;
                        }
                    } else if change == '-' {
                        if c == 'u' {
                            user_bits = 0o0;
                        } else if c == 'g' {
                            group_bits = 0o0;
                        } else if c == 'o' {
                            other_bits = 0o0;
                        }
                    }
                }
            }
        }

        let metadata = fs::metadata(file_path);
        match metadata {
            Ok(metadata) => {
                let mut file_mode = metadata.permissions().mode();

                if change == '+' {
                    file_mode |= user_bits << 6;
                    file_mode |= group_bits << 3;
                    file_mode |= other_bits;
                } else if change == '-' {
                    file_mode &= !(user_bits << 6);
                    file_mode &= !(group_bits << 3);
                    file_mode &= !other_bits;
                }

                let new_permissions = fs::Permissions::from_mode(file_mode);
                match fs::set_permissions(file_path, new_permissions) {
                    Ok(_) => Ok(()),
                    Err(_) => return Err(-25),
                }
            },
            Err(_) => return Err(-25),
        }
    } else {
        if mode.len() == 3 && mode.chars().all(|c| c.is_digit(8)) {
            let parsed_mode = u32::from_str_radix(mode, 8).unwrap();
            let metadata = fs::metadata(file_path);
            match metadata {
                Ok(metadata) => {
                    let mut permissions = metadata.permissions();
                    permissions.set_mode(parsed_mode);
                    match fs::set_permissions(file_path, permissions) {
                        Ok(_) => Ok(()),
                        Err(_) => return Err(-25),
                    }
                },
                Err(_) => return Err(-25),
            }
        } else {
            let mut permissions = 0o0;
            let mut iter = mode.chars();
            let mut user_group = 'u';
    
            while let Some(c) = iter.next() {
                if "ugoa".contains(c) {
                    user_group = c;
                }
    
                for bit in mode.chars() {
                    if bit == 'r' {
                        permissions |= 0o4;
                    }
                    if bit == 'w' {
                        permissions |= 0o2;
                    }
                    if bit == 'x' {
                        permissions |= 0o1;
                    }
                }
    
                let metadata = fs::metadata(file_path);
                match metadata {
                    Ok(metadata) => {
                        let mut file_mode = metadata.permissions().mode();
                        let shift = match user_group {
                            'u' => 6,
                            'g' => 3,
                            'o' => 0,
                            _ => 0,
                        };
                            file_mode &= !(permissions << shift);
            
                        let new_permissions = fs::Permissions::from_mode(file_mode);
                        match fs::set_permissions(file_path, new_permissions) {
                            Ok(_) => (),
                            Err(_) => return Err(-25),
                        }
                    },
                    Err(_) => return Err(-25),
                }
            }
            Ok(())
        }
    }
}


fn main(){

    // TODO 1: Read the command line arguments
    let args: Vec<String> = std::env::args().collect();
    
    // TODO 2: If the first argument is pwd, call pwd()
    if args[1] == "pwd" {
        match pwd() {
            Ok(()) => (),
            Err(()) => (),
        }
    } else if args[1] == "echo"{
        if args[2] == "-n" {
            if args.len() < 5 {
                match echo(&(args[3])) {
                    Ok(_) => (),
                    Err(err) => std::process::exit(err),               
                }
            } else {
                let message = args[3..].join(" ");
                match echo(&message){
                    Ok(_) => (),
                    Err(err) => std::process::exit(err),                
                }
            }
        } else {
            if args.len() < 4 {
                match echo(&(args[2])){
                    Ok(_) => (),
                    Err(err) => std::process::exit(err),                
                }
            } else {
                let message = args[2..].join(" ");
                match echo(&message){
                    Ok(_) => (),
                    Err(err) => std::process::exit(err),                
                }
            }
            print!("\n");
        
        }
    } else if args[1] == "cat"{
        match cat(&args[2..]) {
            Ok(_) => (), 
            Err(err) => std::process::exit(err),
        }
    } else if args[1] == "mkdir" {
        match mkdir(&args[2..]) {
            Ok(_) => (),
            Err(err) => std::process::exit(err),
        }
    } else if args[1] == "mv" {
        match mv(&args[2], &args[3]) {
            Ok(_) => (),
            Err(err) => std::process::exit(err),
        } 
    } else if args[1] == "ln" {
        if args.len() == 5 && (args[2] != "-s" && args[2] != "--symbolic") {
            eprintln!("Invalid command");
            std::process::exit(-1);
        } else {
            match ln(&args[2..]) {
                Ok(_) => (), 
                Err(err) => std::process::exit(err),
            }
        }
    } else if args[1] == "rmdir" {
        match rmdir(&args[2..]) {
            Ok(_) => (),
            Err(err) => std::process::exit(err),
        }
    } else if args[1] == "rm" {
        //nu merge lol :,(
        if args[2].starts_with('-') {
            if  args[2] != "-R" && args[2] != "--recursive" && args[2] != "-d" && args[2] != "--dir"{
                // if {
                    eprintln!("Invalid command");
                    std::process::exit(-1);
                //}
            } else {
                match rm(&args[2..]) {
                    Ok(_) => (),
                    Err(err) => std::process::exit(err),
                }
            }
        } else { 
            match rm(&args[2..]) {
                Ok(_) => (),
                Err(err) => std::process::exit(err),
            }
        }
    } else if args[1] == "ls"{
        match ls(&args[2..]) {
            Ok(_) =>(),
            Err(err) => std::process::exit(err),
        }
    } else if args[1] == "cp" {
        if args[2] == "-R" || args[2] == "-r" || args[2] == "-recusive" {
            match cp (&args[3], &args[4], true) {
                Ok(_) => (),
                Err(err) => std::process::exit(err),
            }
        } else {
            match cp (&args[2], &args[3], false) {
                Ok(_) => (),
                Err(err) => std::process::exit(err),
        }
        }
    } else if args[1] == "touch"{
        match touch(&args[2..]) {
            Ok(_) => (),
            Err(err) => std::process::exit(err),
        }
    } else if args[1] == "chmod" {
        if args[2].contains("-a") {
            eprintln!("Invalid command");
            std::process::exit(-1);
        }
        match chmod(&args[3], &args[2]) {
            Ok(_) => (),
            Err(err) => std::process::exit(err),
        }
    } else {
        eprintln!("Invalid command");
        std::process::exit(-1);
    }
}
