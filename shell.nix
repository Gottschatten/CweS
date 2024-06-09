let 
 pkgs = import <nixpkgs> {config.allowUnfree = true;};
in pkgs.mkShell {
 packages = with pkgs; [
	libgcc
	ncurses
	pkg-config
	openssl
 ];
 PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}
