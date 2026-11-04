const unsigned char zippedBin[] = {
#if __has_embed("../../../build.zip") == __STDC_EMBED_FOUND__
#embed "../../../build.zip"
#else
    0
#endif
};