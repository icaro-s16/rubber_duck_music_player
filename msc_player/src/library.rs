use lofty::{file::{AudioFile, TaggedFile, TaggedFileExt}, tag::Accessor};
use std::{borrow::Cow, fs::{self, File, read_dir}, io};


pub fn get_msc_duration(mut file : File) -> u128{
    let mut time = 0;
    if let Ok(tagged_file) = lofty::read_from(&mut file){
        let properties = tagged_file.properties();
        time = properties.duration().as_millis();
    } 
    time 
}


// Pega as informações da música atual Nome, banda e album
pub fn get_msc_information(mut file : File) -> Vec<String>{
    let mut information_vec = vec![];
    if let Ok(tagged_file) = lofty::read_from(&mut file){
        if let Some(primary_informatio) = tagged_file.primary_tag(){
            information_vec.push(primary_informatio.title().unwrap_or(Cow::default()).to_string());
            information_vec.push(primary_informatio.artist().unwrap_or(Cow::default()).to_string());
            information_vec.push(primary_informatio.album().unwrap_or(Cow::default()).to_string());
        }
    }
    information_vec
}



// Função para listar os arquivos, retornando um Vetor de PathBuf filtrado apena com arquivos sem dir
pub fn msc_files_list(dir_path : &str ) -> io::Result<Vec<File>>{
    Ok(read_dir(dir_path)?
        .filter(|condition| condition.is_ok())
        .map(|entry| fs::File::open(entry.unwrap().path()))
        .filter(|condition| condition.is_ok())
        .map(|entry| entry.unwrap())
        .filter(|file|{
            // Mudar essa implementação do código, está ruim em quesito de 
            // desempenho e possíveis erros
            let mut clone_file = file.try_clone().unwrap();
            check_msc_file_type(&mut clone_file)
        })
        .collect())
    // Some(fs::File::open(entry.unwrap().path()).unwrap())
}

// Função para verificar se o tipo do arquivo é mp3 ou mp4
fn check_msc_file_type(file : &mut File) -> bool{
    let tagged_file = lofty::read_from(file);
    if tagged_file.is_err(){
        return false;
    }
    match tagged_file.unwrap().file_type() {
        lofty::file::FileType::Mp4 | lofty::file::FileType::Mpeg | lofty::file::FileType::Mpc =>  return true,
        _ => return false
    }
}

// Função para transformar os arquivos em tag, para que possam ser pego os metadados
pub fn tagged_file_msc_list(files_list : &mut Vec<File>) -> Vec<TaggedFile>{
    files_list
    .iter_mut()
    .map(|file|{
        lofty::read_from(file)
    })
    .filter(|condition|condition.is_ok())
    .map(|tagged_file| tagged_file.unwrap())
    .collect()
}

// Retorna um Array com o nome das músicas na playlist e numerados 
pub fn playlist_msc_names(dir_path : &str) -> Vec<String>{
    let mut tagged_files = tagged_file_msc_list(&mut msc_files_list(&dir_path[..]).unwrap_or(Vec::new()));
    let mut files_names : Vec<String> =  tagged_files
        .iter_mut()
        .map(|tagged_file|{
            tagged_file.primary_tag()
        })
        .filter(|condition| condition.is_some() )
        .map(|name| name.unwrap().title())
        .filter(|condition| condition.is_some())
        .map(|name| name.unwrap().to_string())
        .collect();
    files_names
        .iter_mut()
        .enumerate()
        .map(|(ref mut index, name)|{
            let mut index_str = String::new();
            *index += 1;
            index_str.push_str(&index.to_string()[..]);
            index_str.push_str(". ");
            name.insert_str(0, &index_str);
            name.clone().to_string()
        })
        .collect()
}
