package io.github.xtrlumen.vbmeta.attestation;

import org.bouncycastle.asn1.ASN1Encodable;
import org.bouncycastle.asn1.ASN1Sequence;

import java.security.cert.CertificateParsingException;

public class AttestationPackageInfo implements java.lang.Comparable<AttestationPackageInfo> {
    private static final int PACKAGE_NAME_INDEX = 0;
    private static final int VERSION_INDEX = 1;

    private final String packageName;
    private final long version;

    public AttestationPackageInfo(String packageName, long version) {
        this.packageName = packageName;
        this.version = version;
    }

    public AttestationPackageInfo(ASN1Encodable asn1Encodable) throws CertificateParsingException {
        if (!(asn1Encodable instanceof ASN1Sequence sequence)) {
            throw new CertificateParsingException(
                    "Expected sequence for AttestationPackageInfo, found "
                            + asn1Encodable.getClass().getName());
        }

        packageName = Asn1Utils.getStringFromAsn1OctetStreamAssumingUTF8(
                sequence.getObjectAt(PACKAGE_NAME_INDEX));
        version = Asn1Utils.getLongFromAsn1(sequence.getObjectAt(VERSION_INDEX));
    }

    public String getPackageName() {
        return packageName;
    }

    public long getVersion() {
        return version;
    }

    @Override
    public String toString() {
        return getPackageName() + " (version code " + getVersion() + ")";
    }

    @Override
    public int compareTo(AttestationPackageInfo other) {
        int res = packageName.compareTo(other.packageName);
        if (res != 0) return res;
        res = Long.compare(version, other.version);
        if (res != 0) return res;
        return res;
    }

    @Override
    public boolean equals(Object o) {
        return (o instanceof AttestationPackageInfo)
                && (0 == compareTo((AttestationPackageInfo) o));
    }
}
