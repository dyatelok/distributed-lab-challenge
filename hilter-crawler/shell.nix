let
  unstable = import <unstable> { };
in
{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell rec {
  # ...
  nativeBuildInputs = [ pkgs.pkg-config ];

  buildInputs = with pkgs; [
    clang
    # Replace llvmPackages with llvmPackages_X, where X is the latest LLVM version (at the time of writing, 16)
    llvmPackages_16.bintools
    rustup
    openssl
    pkg-config
    libgccjit
    bacon
    toybox
    dbus.dev
    unstable.helix
    unstable.nushell
    unstable.starship
    python311Packages.aiohttp
    python311Packages.beautifulsoup4
    python311Packages.lxml
  ];

  # LD_LIBRARY_PATH = lib.makeLibraryPath [ pkgs.openssl ];

  RUSTC_VERSION = pkgs.lib.readFile ./rust-toolchain;
  # https://github.com/rust-lang/rust-bindgen#environment-variables
  LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
  shellHook = ''
    export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
    export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
    export NIX_ENFORCE_PURITY=0
    rustup component add rust-analyzer
    nu
  '';
  # Add precompiled library to rustc search path
  RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
    # add libraries here (e.g. pkgs.libvmi)
  ]);
  # Add glibc, clang, glib and other headers to bindgen search path
  BINDGEN_EXTRA_CLANG_ARGS =
    # Includes with normal include path
    (builtins.map (a: ''-I"${a}/include"'') [
      # add dev libraries here (e.g. pkgs.libvmi.dev)
      pkgs.glibc.dev
    ])
    # Includes with special directory paths
    ++ [
      ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
      ''-I"${pkgs.glib.dev}/include/glib-2.0"''
      ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
    ];
}
