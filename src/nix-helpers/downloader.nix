{ pkgs ? import <nixpkgs> { } }:

let
  source = pkgs.fetchurl {
    url = "{}";
    sha256 = "{}";
  };
in
  unpack = pkgs.runCommandNoCC "{}" { } ''
    mkdir -p $out tmp
    tar xzf ${source} -C tmp
    mv tmp/*/* $out/ # TODO: template file name?
  ''
