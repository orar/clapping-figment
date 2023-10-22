use sqlx::PgPool;
use crate::cli::{Args, ID};
use crate::cli::{Add, Done};
use crate::config::{AppConfig};
use crate::todo::Todos;

pub async fn app(args: Args, config: &AppConfig, pool: &PgPool) -> anyhow::Result<()> {
    match args {
        Args::Add(Add { description }) => {
            println!("Adding new todo with description '{description}'");
            let id = Todos::from(&pool).add(description).await?;
            Ok(println!("Todo added. Id: {}", id))
        },

        Args::Done(Done { id }) => {
            println!("Marking todo {id} as done");
            let id = Todos::from(&pool).complete(id).await?;
            Ok(println!("Todo task completed. #id: {}", id))
        },

        Args::Get(ID{ id }) => {
            let todo = Todos::from(&pool).get(id).await?;
            Ok(println!("Todo #{} {} {}", todo.id, todo.description, todo.done))
        },

        Args::All => {
            let todos = Todos::from(&pool).list().await?;

            if todos.len() < 1 {
                println!("You have no pending tasks to do.")
            } else {
                println!("Printing list of all todos");
                println!("ID -- Description ------- Done");
            }

            for todo in todos.iter() {
                println!("{}, {}, {}", todo.id, todo.description, todo.done)
            }

            Ok(())
        },

        _ => Ok(())
    }
}