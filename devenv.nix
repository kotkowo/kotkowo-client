{pkgs, ...}: {
  dotenv.enable = true;
  languages.rust = {
    enable = true;
    channel = "nightly";
  };
  packages = with pkgs; [openssl];
}
