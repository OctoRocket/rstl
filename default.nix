{ pkgs, rustPlatform, ...}:

rustPlatform.buildRustPackage rec {
    pname = "rstl";
    version = "0.1.0";

    nativeBuildInputs = with pkgs; [ clang mold ];
    src = ./.;

    cargoLock.lockFile = ./Cargo.lock;
}
