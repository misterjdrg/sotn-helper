use clap::Parser;
use regex::Regex;

use core::fmt::Write as _;

use crate::types::Struct;

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
            "entity" => print!("{}", offset(&types::ENTITY, &line)?),
            "prim" => print!("{}", offset(&types::PRIMITIVE, &line)?),
            "enemydef" => print!("{}", offset(&types::ENEMY_DEF, &line)?),
            _ => anyhow::bail!("unknown type"),
        },
    }
    Ok(())
}

fn offset(kind: &Struct, line: &str) -> anyhow::Result<String> {
    if let Some(m) = Regex::new("^(-?[0-9a-fA-F]+)$").unwrap().captures(line)
        && let Some(off) = m.get(1)
        && let Ok(offset) = i32::from_str_radix(off.as_str(), 16)
    {
        return Ok(offset2(kind, "self", offset));
    }
    if let Some(m) = Regex::new("^unk(-?[0-9a-fA-F]+)$").unwrap().captures(line)
        && let Some(off) = m.get(1)
        && let Ok(offset) = i32::from_str_radix(off.as_str(), 16)
    {
        return Ok(offset2(kind, "self", offset));
    }
    if let Some(m) = Regex::new("^([a-zA-Z0-9_]+)->unk(-?[0-9a-fA-F]+)$")
        .unwrap()
        .captures(line)
        && let Some(var) = m.get(1)
        && let Some(off) = m.get(2)
        && let Ok(offset) = i32::from_str_radix(off.as_str(), 16)
    {
        return Ok(offset2(kind, var.as_str(), offset));
    }
    if let Some(m) = Regex::new("^([a-zA-Z0-9_]+)->unk(-?[0-9a-fA-F]+)$")
        .unwrap()
        .captures(line)
        && let Some(var) = m.get(1)
        && let Some(off) = m.get(2)
        && let Ok(offset) = i32::from_str_radix(off.as_str(), 16)
    {
        return Ok(offset2(kind, var.as_str(), offset));
    }
    if let Some(m) = Regex::new("^([a-zA-Z0-9_]+)->(-?[0-9a-fA-F]+)$")
        .unwrap()
        .captures(line)
        && let Some(var) = m.get(1)
        && let Some(off) = m.get(2)
        && let Ok(offset) = i32::from_str_radix(off.as_str(), 16)
    {
        return Ok(offset2(kind, var.as_str(), offset));
    }
    anyhow::bail!("unknown pattern")
}
fn offset2(kind: &Struct, var: &str, offset: i32) -> String {
    let array_off = (offset as f32 / kind.size() as f32).floor() as i32;
    let offset = offset - array_off * kind.size() as i32;

    let var0 = var.to_string();
    let mut var1 = kind.name.to_lowercase();

    let field = kind.locate(offset as usize);

    let mut result = String::new();

    if array_off == 0 {
        var1 = var0;
    } else if array_off > 0 {
        writeln!(&mut result, "{var1} = {var0} + {array_off};").unwrap();
    } else {
        writeln!(&mut result, "{var1} = {var0} - {};", -array_off).unwrap();
    }

    if field.0.2 == "ext" {
        writeln!(&mut result, "{var1}->ext.ILLEGAL.u8[{}]", field.1).unwrap();
    } else {
        if field.1 == 0 {
            writeln!(&mut result, "{var1}->{}", field.0.2).unwrap();
        } else {
            writeln!(&mut result, "{var1}->{}[{}]", field.0.2, field.1).unwrap();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_offset() {
        assert_eq!(offset(&types::ENTITY, "0").unwrap(), "self->posX\n");
        assert_eq!(offset(&types::ENTITY, "10").unwrap(), "self->hitboxOffX\n");
        assert_eq!(
            offset(&types::ENTITY, "-4").unwrap(),
            "entity = self - 1;\nentity->unkB8\n"
        );

        assert_eq!(
            offset(&types::ENTITY, "unk10").unwrap(),
            "self->hitboxOffX\n"
        );
        assert_eq!(
            offset(&types::ENTITY, "self->unk10").unwrap(),
            "self->hitboxOffX\n"
        );
        assert_eq!(
            offset(&types::ENTITY, "aaa->unk10").unwrap(),
            "aaa->hitboxOffX\n"
        );
    }
}
