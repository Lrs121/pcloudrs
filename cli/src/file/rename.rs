use clap::Clap;
use pcloud::file::rename::Params;
use pcloud::http::HttpClient;

#[derive(Clap)]
pub struct Command {
    file_id: usize,
    filename: String,
}

impl Command {
    #[tracing::instrument(skip_all)]
    pub async fn execute(&self, pcloud: HttpClient) {
        let params = Params::new_rename(self.file_id, self.filename.clone());
        match pcloud.rename_file(&params).await {
            Ok(_) => {
                tracing::info!("file renamed");
                std::process::exit(exitcode::OK);
            }
            Err(err) => {
                tracing::error!("unable to rename file: {:?}", err);
                std::process::exit(exitcode::DATAERR);
            }
        }
    }
}
