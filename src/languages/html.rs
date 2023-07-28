use std::fs::File;
use std::io::prelude::*;

pub fn main(project_name: &str, with_js: &str) {
    // Nom du dossier que nous voulons créer
    let main_folder: String = String::from(project_name.clone());
    let css_folder: String = String::from(format!("{}/style", main_folder));
    let js_folder: String = String::from(format!("{}/script", main_folder));
    let html_file: String = String::from(format!("{}/index.html", main_folder));
    let css_file: String = String::from(format!("{}/style/style.css", main_folder));
    let js_file: String = String::from(format!("{}/script/script.js", main_folder));
    let html_content = String::from(format!("<!DOCTYPE html>
<html lang='en'>
    <head>
    <meta charset='UTF-8'>
    <meta name='viewport' content='width=device-width, initial-scale=1.0'>
    <link rel='stylesheet' type='text/css' href='./style/style.css'>
    <title>{}</title>
    </head>
    <body>
    </body>
</html>", project_name));
    let html_js_content = String::from(format!("<!DOCTYPE html>
<html lang='en'>
    <head>
    <meta charset='UTF-8'>
    <meta name='viewport' content='width=device-width, initial-scale=1.0'>
    <link rel='stylesheet' type='text/css' href='./style/style.css'>
    <script type='text/javascript' src='./script/script.js' defer></script>    
    <title>{}</title>
    </head>
    <body>
    </body>
</html>", project_name));
    let css_content = "* {
    margin: 0;
    border: 0;
    padding: 0;    
}
";
    let str_main_folder: &str = main_folder.as_str();
    let str_css_folder = css_folder.as_str();
    let str_js_folder = js_folder.as_str();
    let str_html_file = html_file.as_str();
    let str_html_content = html_content.as_str();
    let str_html_js_content = html_js_content.as_str();
    let str_css_file = css_file.as_str();
    let str_js_file = js_file.as_str();

    if with_js == "y" || with_js == "Y" {
        create_dir(str_main_folder);
        create_dir(str_css_folder);
        create_dir(str_js_folder);
        create_file(str_html_file, str_html_js_content);
        create_file(str_css_file, css_content);
        create_file(str_js_file, "")

    }
    else {
        create_dir(str_main_folder);
        create_dir(str_css_folder);
        create_file(str_html_file, str_html_content);
        create_file(str_css_file, css_content);
    }
    


    
}

fn create_dir(folder: &str) {
    match std::fs::create_dir(folder) {
        Ok(_) => println!("Le dossier {} a été créé avec succès.", folder),
        Err(e) => {
            println!("Erreur lors de la création du dossier {} : {}", folder, e);
            return;
        }
    }
}

fn create_file(file: &str, content: &str) {
    match File::create(&file) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(content.as_bytes()) {
                println!("Erreur lors de l'écriture dans le fichier {}", e);
            } else {
                println!("Le fichier a été créé avec succès.");
            }
        }
        Err(e) => {
            println!("Erreur lors de la création du fichier {} : {}", file, e);
        }
    }
}