let
   pkgs = import <nixpkgs> {};
in pkgs.stdenv.mkDerivation rec {
  name = "bullet-hex";
  buildInputs = [ pkgs.cargo pkgs.SDL2 ];
}
