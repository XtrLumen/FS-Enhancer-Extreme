package io.github.xtrlumen.vbmeta.attestation;

class EatClaim {
    public static final int BOOT_STATE = -76003;

    private static final int PRIVATE_BASE = -80000;
    private static final int NON_KM_BASE = PRIVATE_BASE - 2000;

    public static final int VERIFIED_BOOT_KEY = NON_KM_BASE - 1;
    public static final int DEVICE_LOCKED = NON_KM_BASE - 2;
    public static final int VERIFIED_BOOT_HASH = NON_KM_BASE - 3;
    public static final int OFFICIAL_BUILD = NON_KM_BASE - 6;
}
