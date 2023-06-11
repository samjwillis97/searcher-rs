{ pkgs, lib, ... }: {
  packages = (with pkgs; [ rustup ]);

  # packages = (with pkgs; [ nodePackages.pnpm clang ])
  #   ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin;
  #     [
  #       libiconv
  #       #      apple_sdk.frameworks.AppKit
  #       #      apple_sdk.frameworks.Cocoa
  #       #      apple_sdk.frameworks.CoreFoundation
  #       #      apple_sdk.frameworks.WebKit
  #       #      apple_sdk.frameworks.Security
  #       #      libobjc
  #     ]);

  # env.LD_LIBRARY_PATH = "$DEVENV_PROFILE/lib:$LD_LIBRARY_PATH";

  # env.CC = "${pkgs.clang}/bin/clang";

  # https://devenv.sh/languages/
  # languages.nix.enable = true;

  # https://devenv.sh/pre-commit-hooks/
  # pre-commit.hooks.shellcheck.enable = true;

  # https://devenv.sh/processes/
  # processes.ping.exec = "ping example.com";

  # See full reference at https://devenv.sh/reference/options/
}
