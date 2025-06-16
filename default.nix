{pkgs ? import <nixpkgs> {}}: let
  external_binaries = with pkgs; [
    fzf
    fd
  ];
in
  pkgs.stdenv.mkDerivation rec {
    pname = "zelegate";
    version = "0.2.1";

    src = ./.;

    nativeBuildInputs = with pkgs; [
      nim
      makeWrapper
    ];

    buildPhase = ''
      export XDG_CACHE_HOME=$(mktemp -d)
      nim c -d:release --out:zelegate ./zelegate.nim
      nim c -d:release --out:zelewidget ./zelewidget.nim
    '';

    installPhase = ''
      mkdir -p $out/bin
      install -Dm755 zelegate $out/bin
      install -Dm755 zelewidget $out/bin

      wrapProgram $out/bin/zelegate --prefix PATH : "${pkgs.lib.makeBinPath external_binaries}"
    '';
  }
