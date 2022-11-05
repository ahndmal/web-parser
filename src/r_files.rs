use std::fs;

fn read_lorems() {
    let dir_name = "/home/andriim/Documents/";
    let dir = fs::read_dir(dir_name);
    match dir {
        Ok(dir) => println!("{:?}", {
            dir.for_each(|item| {
                // println!("{:?}", item.unwrap().file_name());
                let f_name = item.unwrap().file_name();
                let file_path = format!("{}{}", dir_name,f_name.to_str().unwrap().to_string());
                match fs::read_to_string(file_path) {
                    Ok(data) => println!("{:?}", data),
                    Err(err) => println!("{:?}", err)
                }
                // let file_res = File::open(file_path);
                // match file_res {
                //     Ok(file) => {println!("{:?}", file)},
                //     Err(err) => {println!("{:?}", err)}
                // }

            });
        }),
        Err(err) => println!("{err}"),
    }
}