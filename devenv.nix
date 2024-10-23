{ pkgs
, lib
, config
, inputs
, ...
}:
{
  # https://devenv.sh/basics/
  env = {
    PROJECT = "neuronek-cli";
    RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
    RUST_BACKTRACE = "full";
    CARGO_LOG = "warn";
    SCCACHE_LOG = "warn";
  };

  dotenv = {
    enable = true;
    disableHint = true;
  };

  devenv = {
    debug = false;
    warnOnNewVersion = false;
  };

  # https://devenv.sh/packages/
  packages = with pkgs; [
    git
    rustup
    openssl
    onefetch
    direnv
    nix-direnv
    nix-direnv-flakes
    sccache
    buck2
    ccache
    adrgen
    cargo-temp
    cargo-chef
    cargo-vet
    cargo-make
    cargo-cross
    cargo-binstall
    cargo-bundle
    cargo-cranky
    cargo-msrv
    cargo-zigbuild
    cargo-nextest
    cargo-dist
    cargo-xwin
    cargo-xbuild
    cargo-bundle
    cargo-deb
    cargo-expand    
    earthly
    cargo-autoinherit
    rustfilt
    sqlite
    llvmPackages.bintools
          llvmPackages_latest.llvm
      llvmPackages_latest.bintools
      zlib.out
      rustup
      xorriso
      grub2
      qemu
      llvmPackages_latest.lld
      python3
      makeself
      upx
      candle
      dotnet-sdk
      dotnet-runtime           
      bacon   
      ];

  # https://devenv.sh/languages/
  languages = {
    rust = {
      enable = true;
      channel = "nightly";
      mold.enable = true;
      components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" "rust-std" "rust-src" "llvm-tools" "rust-docs" "llvm-tools-preview" ];
      targets = [
      "x86_64-unknown-linux-gnu"
      "x86_64-unknown-linux-musl"
      "x86_64-pc-windows-gnu"
      "x86_64-pc-windows-msvc"
      "x86_64-apple-darwin"
      "aarch64-apple-darwin"
      "aarch64-unknown-linux-musl"
      "aarch64-pc-windows-msvc"
      ];
    };
    zig = {
      enable = true;
    };
  };

  # https://devenv.sh/processes/
   processes.cargo-watch.exec = "cargo-watch";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/scripts/
  scripts.motd.exec = "onefetch";
  scripts.lint.exec = ''
   cargo clippy --fix
   cargo fmt --all
  '';
  scripts.build.exec = "cargo build";
  scripts.test.exec = "cargo test";
  scripts.dev.exec = "bacon";
  scripts.nd.exec = "cargo run";

  enterShell = ''
    motd
  '';

  # https://devenv.sh/tests/
  enterTest = ''
  '';

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
