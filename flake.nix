 {
   description = "Base";
 
   inputs = {
     nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable" ;
   };
 
   outputs = { self, nixpkgs }:
 
     let
       system = "aarch64-darwin";
       pkgs = nixpkgs.legacyPackages.${system};
 
     in
     {
       devShells.${system}.default =
         pkgs.mkShell {
           packages = with pkgs; [
             cargo
	     rustc
	     libiconv
           ];
	   buildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin
	      (with pkgs.darwin.apple_sdk.frameworks; [
		AppKit
	      ]);
         };
     };
 }
