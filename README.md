# clapping-figment
A minimal cli Todo app using Clap, Figment, Sqlx(postgres) 

*Nothing but a clapping figment. All I see but a shrouded config*


1. Edit the config.toml & env.sh file with database url
2. Run `source env.sh` to export DATABASE_URL into enviroment
3. Run `cargo check`
4. Run `cargo build`
5. Bundled rust application is found in `target/debug/clapping-figment`. 
Run setup using `target/debug/clapping-figment setup`.
to run database migrations.