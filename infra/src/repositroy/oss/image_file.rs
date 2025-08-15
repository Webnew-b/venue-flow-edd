use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};

use minio::s3::args::PutObjectArgs;
use minio::s3::client::Client;


pub struct ImagePath<'a>{
    pub bucket_name:String,
    pub image_path:&'a Path,
}

pub fn gen_uuid_image_name(ext:&str) -> String {
    let id = uuid::Uuid::new_v4();
    format!("image_{}.{}",id,ext)
}

pub fn change_image_type(image_path:&Path,output:&Path)-> Result<(),io::Error> {
    let img = image::open(image_path).map_err(|e|{
        tracing::error!("{}",e.to_string());
        io::Error::new(io::ErrorKind::InvalidData,"Could not open the image file.")
    })?;

    img.save_with_format(output, image::ImageFormat::WebP)
        .map_err(|e|{
            tracing::error!("{}",e.to_string());
            io::Error::new(io::ErrorKind::InvalidData,"Could not create the image file.")
        })
}


fn change_path_extension(source_path:&Path)-> io::Result<PathBuf> {
    let parent_path = match source_path.parent() {
      Some(p) => p,
      None => {
          return Err(
              io::Error::new(
                  io::ErrorKind::InvalidData,
                  "Could not get the parent path"
                  )
              );
      }
    };
    let mut file_name = match source_path.file_stem() {
        Some(p) => p.to_string_lossy().to_string(),
        None => {
          return Err(
              io::Error::new(
                  io::ErrorKind::InvalidData,
                  "Could not get the file name"
                  )
              );
      }
    };
    file_name.push_str(".webp");

    Ok(parent_path.join(file_name))
}

pub async fn save_file_to_oss<'a>(client:&Client,path:ImagePath<'a>) -> Result<String,io::Error> {

    let output = change_path_extension(path.image_path)?;

    let _ = change_image_type(path.image_path, output.as_path());

    let file_stream = File::open(output.clone())?;
    let file_meta = fs::metadata(output.clone())?;
    let file_size = Some(file_meta.len() as usize);

    let output_c = output.clone();
    let file_name = match output_c.file_name(){
        Some(e)=>e.to_string_lossy(),
        None => {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Could not get the file name"));
        }
    };

    let mut box_file_stream = Box::new(file_stream);

    let mut po = PutObjectArgs::new(
        &path.bucket_name, 
        &file_name, 
        &mut box_file_stream, 
        file_size, 
        None)
        .map_err(|e|{
            tracing::error!("{}",e.to_string());
            io::Error::new(io::ErrorKind::Interrupted,"Fail to instructure the object args")
        })?;

    let _ = client.put_object(&mut po).await
        .map_err(|e|{ 
            tracing::error!("{}",e.to_string());
            io::Error::new(io::ErrorKind::ConnectionAborted,"Fail to upload the file to oss")
        })?;

    let delete_cache_file = path.image_path.to_path_buf();
    send2queue(delete_cache_file).await;
    send2queue(output).await;
        
    Ok(file_name.to_string())

}

async fn send2queue(p:PathBuf) {
    let queue = DETELE_FILE_QUEUE.clone();
    let sender = &queue.0;
    if let Err(e) = sender.send(p).await {
        tracing::error!("{}",e.to_string());
    }
}
