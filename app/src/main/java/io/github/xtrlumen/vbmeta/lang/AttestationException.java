package io.github.xtrlumen.vbmeta.lang;

public class AttestationException extends RuntimeException {

    public static final int CODE_UNKNOWN = -1;
    public static final int CODE_UNAVAILABLE = 0;
    public static final int CODE_CANT_PARSE_CERT = 2;
    public static final int CODE_STRONGBOX_UNAVAILABLE = 3;
    public static final int CODE_DEVICEIDS_UNAVAILABLE = 4;
    public static final int CODE_OUT_OF_KEYS = 5;
    public static final int CODE_OUT_OF_KEYS_TRANSIENT = 6;
    public static final int CODE_UNAVAILABLE_TRANSIENT = 7;
    public static final int CODE_KEYS_NOT_PROVISIONED = 8;
    public static final int CODE_RKP = 9;

    private final int code;

    public AttestationException(int code, Throwable cause) {
        super(cause);
        this.code = code;
    }

    public int getCode() {
        return code;
    }

    public int getTitleResId() {
        return 0;
    }

    public int getDescriptionResId() {
        return 0;
    }

    @Override
    public Throwable fillInStackTrace() {
        return this;
    }
}
