use std::path::PathBuf;

use super::CargoResult;
use anyhow::{bail, Context};

pub enum Download {
    Registry{
        url: String,
        hash: String,
        descriptor: String,
    },
}

impl Download {
    pub fn registry(url: String, descriptor: String, hash: String) -> CargoResult<Self> {
        if hash.len() != 64 {
            bail!("Not a sha256 sum error")
        }

        Ok(Download::Registry {
            url, descriptor, hash
        })
    }

    fn expr_registry(&self, unpack: bool) -> CargoResult<String> {
        if let Download::Registry{url, descriptor, hash} = self {
          if hash.len() != 64 {
              bail!("Not a sha256 sum error")
          }
          let name = descriptor.split(' ').next().context("could not get name from descriptor")?;

          let vars = format!("let pkgs = import <nixpkgs> {{}}; crate = pkgs.fetchurl {{ url = \"{}\"; sha256 = \"{}\"; name = \"{}.crate\"; }}; in", url, hash, name);

          Ok(match unpack {
              true => format!("{} pkgs.runCommandNoCC \"{}\" {{}} \"mkdir -p $out; tar xvf ${{crate}} -C $out --strip-components=1\"", vars, name),
              false => format!("{} crate", vars),
          })
        } else {
            bail!("BUG: not a Registry derivation");
        }
    }

    pub fn expr(&self, unpack: bool) -> CargoResult<String> {
        match self {
            Download::Registry{ .. } => self.expr_registry(unpack)
        }
    }

    /// downloads and unpacks a package via nix
    pub fn build(&self) -> CargoResult<PathBuf> {
        let expr = self.expr(true)?;
        println!("expr: {}", expr);

        println!("TODO: implement nix build call");
        //Ok(PathBuf::from("/home/kloenk/.cargo/registry/src/github.com-1ecc6299db9ec823/aho-corasick-0.7.13/"))

        todo!("realize")
    }

}
