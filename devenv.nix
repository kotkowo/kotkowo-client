{pkgs, ...}: {
  dotenv.enable = true;
  languages.rust = {
    enable = true;
    channel = "nightly";
  };
  packages = with pkgs; [openssl cargo-insta vscode-extensions.vadimcn.vscode-lldb.adapter];
  enterShell = ''
    export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:"$DEVENV_PROFILE/lib/"
  '';
}
