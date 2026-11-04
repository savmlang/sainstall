// ZIP Payload
const unsigned char zippedBin[] = {
#if __has_embed("../../../build.zip") == __STDC_EMBED_FOUND__
#embed "../../../build.zip"
#else
    0
#endif
};
const size_t zippedBinLen = sizeof(zippedBin);

// License Text
const unsigned char licenseText[] = {
#embed "../../../LICENSE" suffix(, 0)
};
const size_t licenseTextLen = sizeof(licenseText);