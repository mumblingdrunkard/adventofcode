{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  inputsFrom = [ (pkgs.callPackage ./default.nix { }) ];

  buildInputs = with pkgs; [ rust-analyzer rustfmt clippy watchexec vscode-extensions.llvm-org.lldb-vscode ];
}
