use rust_bert::pipelines::translation::{Language, TranslationModelBuilder};
use std::fs::File;
use std::io:: Read;

pub fn read_file(path:String)->anyhow::Result<String>{
    let mut file=File::open(path)?;
    let mut contents=String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}


pub fn read_file_array(path:String )->anyhow::Result<Vec<String>>{
    let mut file:File= File::open(path)?;
    let mut contents: String=String ::new();

    file.read_to_string(buf:&mut contents)?;
    let array:Vec<String>=contents.lines().map(|s:&str| s.to_string()).collect();
    Ok(array)
}

pub fn translate_file(path:String)->anyhow::Result<()>{
    let model:TranslationModel=TranslationModelBuilder::new() TranslationModelBuilder
        .with_source_language(vec![Language::English]) &mut TranslationModelBuilder
        .with_target_language(vec![Language::French]) &mut TranslationModelBuilder
        .create_model()?;
        let text :Vec<String>=read_file_array(path)?;
        let output :Vec<String>=model.translate(texts:&text , source_language:None, target_language:Language::French)
        for sentence in output {
            println!("{}", sentence);
        }
        Ok(())
    
}