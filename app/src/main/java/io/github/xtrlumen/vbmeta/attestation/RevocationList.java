package io.github.xtrlumen.vbmeta.attestation;

import java.math.BigInteger;

public record RevocationList(String status, String reason) {
    public static RevocationList get(BigInteger serialNumber) {
        return null;
    }

    @Override
    public String toString() {
        return "status is " + status + ", reason is " + reason;
    }
}
