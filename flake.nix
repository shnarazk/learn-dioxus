{
  description = "A Fukuoka COVID-19 stats viewer in Rust/Dioxus";
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs;
    flake-utils.url = github:numtide/flake-utils;
  };
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: {
      defaultPackage =
        with import nixpkgs { system = "${system}"; };
        stdenv.mkDerivation rec {
          name = "fukuoka-covid19-${version}";
          pname = "fukuoka-covid19";
          version = "0.1.0";
          src = self;
          buildInputs = rustc.buildInputs ++ [ cargo rustc ] ++
                        lib.optionals stdenv.isDarwin (
                          with darwin.apple_sdk.frameworks; [ AppKit Carbon WebKit ]);
          buildPhase = "cargo build --release";
          installPhase = ''
              mkdir -p $out/bin;
              cp target/release/learn-dioxus $out/bin/fukuoka-covid19
          '';
        }
      ;
    })
  ;
}
