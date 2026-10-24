{
    description = "Environment to build Rust projects (rattrade)";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    };

    outputs = {self, nixpkgs, ... }:
        let
            system = "x86_64-linux";
            pkgs = nixpkgs.legacyPackages.${system};
        in
        {
            devShells.${system}.default = pkgs.mkShell {
                packages = with pkgs; [
                    git
                    cargo
                    rustc
                    rustfmt
                    clippy
                    rust-analyzer
                    openssl.dev
                    dbeaver-bin
                    pkg-config
                    cloc
                ];

                shellHook = ''
                    echo "Welcome to your reproducible development environment!"
                    export PS1="[norka-terminal-dev] $PS1"
                    export PATH="$PATH:/home/thereactivemouse/.cargo/bin"
                '';
            };
        };
}
