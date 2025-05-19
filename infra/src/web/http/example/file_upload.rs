use std::{fs, io};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;
use actix_web::{post, web, HttpResponse};
use image::{ImageFormat, ImageReader};
use serde::{Deserialize, Serialize};

use crate::repositroy::image_file::{gen_uuid_image_name, save_file_to_oss, ImagePath};
use crate::web::http::{CustomResponse, CustomResponseError};
use crate::web::AppState;

#[derive(Debug,MultipartForm)]
struct Upload {
    #[multipart(limit="10MB")]
    pub file:TempFile,
}

fn verify_image_type(path:&Path) -> io::Result<()> {

    if !path.exists() { 
        let e = io::Error::new(io::ErrorKind::Interrupted,"File is not found.");
        return Err(e);
    }

    let reader = ImageReader::open(path).map_err(|e|{
        tracing::error!("{}",e.to_string());
        io::Error::new(io::ErrorKind::Interrupted,"Could not read the image")
    })?;
    let reader = reader.with_guessed_format().map_err(|e|{
        tracing::error!("{}",e.to_string());
        io::Error::new(io::ErrorKind::InvalidData, "Image type is invaild")
    })?;
    let fmt = reader.format();

    match fmt {
        Some(ImageFormat::Png) | Some(ImageFormat::Jpeg) | Some(ImageFormat::Gif) => Ok(()),
        _ => {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Image type is invaild"))
        }
    }
    
}

#[post("/upload")]
pub async fn example_upload_oss(
        config:web::Data<AppState>,
        MultipartForm(form):MultipartForm<Upload>
    ) -> Result<HttpResponse,CustomResponseError> {

    let config = &config.oss;
    
    let temp_file_path = form.file.file.path();

    if let Err(e) = verify_image_type(&temp_file_path) {
        tracing::error!("{}",e.to_string());
        return Err(CustomResponseError::BadRequest("The file format is not a image.".to_string()));
    }

    let temp_file_name = form.file.file_name;

    let temp_file_name = match temp_file_name {
        Some(n) => n,
        None => {
            return Err(CustomResponseError::BadRequest("The file name is invaild.".to_string()));
        }
    };

    let ext = Path::new(&temp_file_name)
        .extension()
        .and_then(|ext| ext.to_str());
        
    let ext = match ext {
        Some(n) => n,
        None => {
            return Err(CustomResponseError::BadRequest("The file extension is invaild.".to_string()));
        }
    };


    let file_name :&str = &gen_uuid_image_name(ext);

    let mut file_path = match PathBuf::from_str(&config.get_temp_folder()) {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("Could not get the file path,cause:{}",e.to_string());
            return Err(CustomResponseError::ServiceError);
        }
    };

    file_path.push(file_name);

    if let Err(e) = fs::copy(temp_file_path,file_path.clone()) {
        tracing::error!("Could not move the file,cause:{}",e.to_string());
        return Err(CustomResponseError::ServiceError);
    }

    let image_path = ImagePath {
        image_path:file_path.as_path(),
        bucket_name:config.get_bucket_name().to_string()
    };

    let uri = match save_file_to_oss(&config.get_client(), image_path).await {
        Ok(e) => e,
        Err(e)=>{
            tracing::error!("{}",e.to_string());
            return Err(CustomResponseError::ServiceError);
        }
    };

    let uri = format!("{}{}",config.get_image_domain(),uri);

    #[derive(Deserialize,Serialize)]
    struct Res {
        image_url:String
    }
    
    let res = CustomResponse::<Res>::success_by_response(Some(Res{
        image_url:uri
    }));
    Ok(res)
}
