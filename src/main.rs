use clap::Parser;

mod offset;
mod types;

#[derive(clap::Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(clap::Subcommand)]
enum Cmd {
    Offset { kind: String, line: String },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.cmd {
        Cmd::Offset { kind, line } => match kind.to_ascii_lowercase().as_str() {
            "entity" => print!("{}", offset::call(&types::ENTITY, &line)?),
            "prim" => print!("{}", offset::call(&types::PRIMITIVE, &line)?),
            "enemydef" => print!("{}", offset::call(&types::ENEMY_DEF, &line)?),
            _ => anyhow::bail!("unknown type"),
        },
    }
    Ok(())
}
