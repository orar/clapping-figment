use clap::Parser;



/// A minimal todo app using clap, figment, tokio & sqlx(postgres)
#[derive(Parser, Debug)]
pub enum Args {
    All,
    Add(Add),
    Done(Done),
    Get(ID),

    Setup
}

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Add {
    #[arg(short, long)]
    pub description: String
}

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Done {
    #[arg(short, long)]
    pub id: i32
}

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ID {
    #[arg(short, long)]
    pub id: i32
}

