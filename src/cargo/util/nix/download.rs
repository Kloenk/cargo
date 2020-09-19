use std::path::PathBuf;
use std::process::Command;

use super::CargoResult;
use super::super::Config;
use anyhow::{bail, Context};

pub struct Download {
    inner: InnerDownload,
    //hash: String,
    descriptor: String,
}

enum InnerDownload {
    Url{
        url: String,
        hash: String,
    },
}

impl InnerDownload {
    /// returns if the Crate source send the crate in an archive
    pub fn packed(&self) -> bool {
        match self {
            InnerDownload::Url{ .. } => true,
        }
    }

    pub fn expr(&self, descriptor: &str) -> CargoResult<String> {
        match self {
            InnerDownload::Url{ url, hash } => {
              let (name, _) = descripe(descriptor)?;
              Ok(format!("let pkgs = import <nixpkgs> {{}}; cratePacked = pkgs.fetchurl {{ url = \"{}\"; sha256 = \"{}\"; name = \"{}.crate\"; }}; crate = pkgs.runCommandNoCC \"{}\" {{}} \"mkdir -p $out; tar xvf ${{cratePacked}} -C $out --strip-components=1; echo -n ok > $out/.cargo-ok\"; in ", url, hash, name, name))
            },
        }
    }
}

fn descripe(descriptor: &str) -> CargoResult<(String, String)> {
          let descriptor: Vec<&str> = descriptor.split(' ').collect();
          if descriptor.len() != 2 {
              bail!("BUG: descriptor does not have to elements");
          }
          Ok((descriptor[0].to_string(), descriptor[1][1..].to_string()))
}

impl Download {
    pub fn registry(url: String, descriptor: String, hash: String) -> CargoResult<Self> {
        if hash.len() != 64 {
            bail!("Not a sha256 sum error")
        }

        Ok(Self {
            descriptor,
            inner: InnerDownload::Url{ url, hash },
        })
    }


    pub fn expr(&self, unpack: bool) -> CargoResult<String> {
        if !self.inner.packed() && unpack {
            bail!("BUG: cannot unpack a non packed crate");
        }
        let mut expr = self.inner.expr(&self.descriptor)?;

        if unpack {
            expr.push_str("crate");
        } else {
            expr.push_str("cratePacked");
        }

        Ok(expr)
    }

    /// downloads and unpacks a package via nix
    pub fn build(&self, config: &Config) -> CargoResult<PathBuf> {
        let expr = self.expr(self.inner.packed())?;
        println!("expr: {}", expr);

        println!("TODO: implement nix build call");
        let (name, version) = descripe(&self.descriptor)?;
        let path = config.registry_source_path().join(format!("{}-{}", name, version));
        println!("path: {}", path.display());

        let mut cmd = Command::new("nix")
            .arg("build")
            .arg("--impure")
            .arg("--out-link")
            .arg(path.display().to_string())
            .arg("--expr")
            .arg(expr)
            .spawn()?;

        cmd.wait()?;

        //Ok(PathBuf::from("/home/kloenk/.cargo/registry/src/github.com-1ecc6299db9ec823/aho-corasick-0.7.13/"))

        Ok(path.into_path_unlocked())
    }

}
