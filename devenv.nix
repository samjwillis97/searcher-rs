{ pkgs, lib, ... }: {
  languages.rust = {
    enable = true;
    # https://devenv.sh/reference/options/#languagesrustchannel
    channel = "stable";

    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };
}
