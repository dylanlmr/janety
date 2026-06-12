{
  pkgs,
  ...
}:

let
  janet_latest = pkgs.janet.overrideAttrs (old: rec {
    version = "1.41.2";
    src = pkgs.fetchFromGitHub {
      owner = "janet-lang";
      repo = "janet";
      rev = "v${version}";
      sha256 = "sha256-sNRhcGG8JysmPHHXeRkYCt7qA75U6flptUEWWun+rDs=";
    };
  });

  spork_latest = pkgs.stdenv.mkDerivation rec {
    pname = "spork";
    version = "1.2.0";

    src = pkgs.fetchFromGitHub {
      owner = "janet-lang";
      repo = "spork";
      rev = "v${version}";
      sha256 = "sha256-aAM9USwh3ZifupHVPqu/aFyaLrTGlYnzV/88RDkpLjE=";
    };

    nativeBuildInputs = [
      janet_latest
      pkgs.gcc
    ];

    installPhase = ''
      mkdir -p $out/lib/janet $out/bin

      export JANET_PATH=$out/lib/janet
      export JANET_MODPATH=$out/lib/janet
      export JANET_LIBPATH=$out/lib/janet
      export JANET_BINPATH=$out/bin

      janet --install .

      rm -rf $out/lib/janet/bundle
    '';
  };

  janet_modules = pkgs.symlinkJoin {
    name = "janet-modules-env";
    paths = [ spork_latest ];
  };
in
{
  languages.rust = {
    enable = true;
    toolchainFile = ./rust-toolchain.toml;
  };

  packages = [
    janet_latest
    spork_latest
    pkgs.git
    pkgs.gcc
  ];

  env = {
    JANET_PATH = "${janet_modules}/lib/janet";
  };
}
