use std::path::PathBuf;

use super::CargoResult;
use anyhow::{bail, Context};

pub struct Download {
    pub url: String,
    pub hash: String,
    pub description: String,

}

impl Download {
    pub fn url(url: String, description: String, hash: String) -> CargoResult<Self> {
        if hash.len() != 64 {
            bail!("Not a sha256 sum error")
        }

        Ok(Download {
            url, description, hash
        })
    }

    pub fn expr(&self, unpack: bool) -> CargoResult<String> {
        if self.hash.len() != 64 {
            //bail!("Not a sha256 sum error")
        }
        let name = self.description.split(' ').next().context("could not get name from descriptor")?;

        let vars = format!("let pkgs = import <nixpkgs> {{}}; crate = pkgs.fetchurl {{ url = \"{}\"; sha256 = \"{}\"; name = \"{}.crate\"; }}; in", self.url, self.hash, name);

        Ok(match unpack {
            true => format!("{} pkgs.runCommandNoCC \"{}\" {{}} \"mkdir -p $out; tar xvf ${{crate}} -C $out --strip-components=1\"", vars, name),
            false => format!("{} crate", vars),
        })
    }

    /// downloads and unpacks a package via nix
    pub fn build(&self) -> CargoResult<PathBuf> {
        let expr = self.expr(true)?;
        println!("expr: {}", expr);

        todo!("realize")
    }

}