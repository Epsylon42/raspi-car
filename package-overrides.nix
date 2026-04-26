final: prev: 
{
  makeModulesClosure = x: # linux kernel won't build without this
    prev.makeModulesClosure (x // { allowMissing = true; });
  libtheora = prev.libtheora.overrideAttrs (old: {
    configureFlags = (old.configureFlags or []) ++ [ "--disable-asm" ];
  });
  v4l-utils = prev.v4l-utils.override (old: {
    withGUI = false;
  });
}
