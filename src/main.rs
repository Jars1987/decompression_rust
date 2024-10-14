use std::fs;
use std::io;

fn main() {
    std::process::exit(real_main());
}

fn real_main () -> i32 {
let args : Vec<_> = std::env::args().collect();

if args.len() < 2 {
    println!("To decompress provide the following command: {} <filename>", args[0]);
    return 1;
}

let file_name = std::path::Path::new(&args[1]);

let file = match fs::File::open(&file_name) {
    Ok(file) => file,
    Err(err) => {
        println!("Error! Can't find the file name provided");
        return 1;
    }
};

let mut archive = match zip::ZipArchive::new(file){
    Ok(zip) => zip,
    Err(e) => {
        println!("Error! Can't crompress the selected file");
        return 1;
    }
};

for i in 0..archive.len() {

    // loop through archive and get the files
    let mut file = match archive.by_index(i){
        Ok(file) => file,
    Err(err) => {
        println!("Error! Can't retrive the file {} in the archive", i);
        return 1;
    }
    };

    //Outpath for that file retrieved
    let outpath = match file.enclosed_name() {
        Some(path) => path.to_owned(),
        None => continue
    };

    //Check for comments
    {
        let comment = file.comment();
        if !comment.is_empty() {
            println!("File {} comment: {}", i, comment);
        }
    }

    //Cech of the file is directory 
    if (*file.name()).ends_with('/'){
        print!("File {} extracted to: \"{}\"", i, outpath.display());
        match fs::create_dir_all(&outpath){
            Ok(()) => continue,
           Err(err) => {
                println!("Error! Unable to create directory to decompress the file");
                return 1;
            }
        }
    } else {
        print!("File {} extracted to: \"{}\" ( {} bytes)", i, outpath.display(), file.size());

        //If not a directory then check if parent file already created
        if let Some(p) = outpath.parent(){
            if !p.exists(){
                match fs::create_dir_all(&p){
                    Ok(()) => continue,
                   Err(err) => {
                        println!("Error! Unable to create directory to decompress the file");
                        return 1;
                    }
                }
            }
        }

        // create the newfile and copy the file to the newfile
        let mut outfile = fs::File::create(&outpath).unwrap();
        io::copy(&mut file, &mut outfile).unwrap();
    }

    //Add permissions, take ownership of the file if you don't have them otherwise after copying the files you wont be able to read them or do anything
      #[cfg(unix)]
      {
        use std::os::unix::fs::PermissionsExt;

        if let Some(mode) = file.unix_mode() {
            fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
        }
      }
    };
    0 
}
