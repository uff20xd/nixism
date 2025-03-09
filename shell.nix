{pkgs ? import <nixpkgs> {}}: 
let
  
in pkgs.mkShell {

    buildInputs = with pkgs; [
      cargo
      rustc 
    ];

    shellHook = ''
      alias nixism="steam-run ~/programming/nixism/target/debug/nixism"
    '';

}
