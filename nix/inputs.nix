{ pkgs, ... }:

let
  buildInputs = with pkgs; [
    pkg-config
    glib
    cairo
    pango
    gdk-pixbuf
    atkmm
    gtk3
  ];
in
{
  inherit buildInputs;
}