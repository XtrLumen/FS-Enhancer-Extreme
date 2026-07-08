/*
 * Copyright (C) 2016 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
 * in compliance with the License. You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software distributed under the License
 * is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
 * or implied. See the License for the specific language governing permissions and limitations under
 * the License.
 */

package io.github.xtrlumen.vbmeta.attestation;

import org.bouncycastle.asn1.ASN1Encodable;
import org.bouncycastle.asn1.ASN1Sequence;
import org.bouncycastle.asn1.ASN1Set;

import java.security.cert.CertificateParsingException;
import java.util.ArrayList;
import java.util.List;

public class AttestationApplicationId {
    private static final int PACKAGE_INFOS_INDEX = 0;
    private static final int SIGNATURE_DIGESTS_INDEX = 1;

    private final List<AttestationPackageInfo> packageInfos;
    private final List<byte[]> signatureDigests;

    public AttestationApplicationId(ASN1Encodable asn1Encodable)
            throws CertificateParsingException {
        if (!(asn1Encodable instanceof ASN1Sequence sequence)) {
            throw new CertificateParsingException(
                    "Expected sequence for AttestationApplicationId, found "
                            + asn1Encodable.getClass().getName());
        }

        packageInfos = parseAttestationPackageInfos(sequence.getObjectAt(PACKAGE_INFOS_INDEX));
        packageInfos.sort(null);
        signatureDigests = parseSignatures(sequence.getObjectAt(SIGNATURE_DIGESTS_INDEX));
        signatureDigests.sort(new ByteArrayComparator());
    }

    private List<AttestationPackageInfo> parseAttestationPackageInfos(ASN1Encodable asn1Encodable)
            throws CertificateParsingException {
        if (!(asn1Encodable instanceof ASN1Set set)) {
            throw new CertificateParsingException(
                    "Expected set for AttestationApplicationsInfos, found "
                            + asn1Encodable.getClass().getName());
        }

        List<AttestationPackageInfo> result = new ArrayList<AttestationPackageInfo>();
        for (ASN1Encodable e : set) {
            result.add(new AttestationPackageInfo(e));
        }
        return result;
    }

    private List<byte[]> parseSignatures(ASN1Encodable asn1Encodable)
            throws CertificateParsingException {
        if (!(asn1Encodable instanceof ASN1Set set)) {
            throw new CertificateParsingException("Expected set for Signature digests, found "
                    + asn1Encodable.getClass().getName());
        }

        List<byte[]> result = new ArrayList<byte[]>();

        for (ASN1Encodable e : set) {
            result.add(Asn1Utils.getByteArrayFromAsn1(e));
        }
        return result;
    }

    private static class ByteArrayComparator implements java.util.Comparator<byte[]> {
        @Override
        public int compare(byte[] a, byte[] b) {
            int res = Integer.compare(a.length, b.length);
            if (res != 0) return res;
            for (int i = 0; i < a.length; ++i) {
                res = Byte.compare(a[i], b[i]);
                if (res != 0) return res;
            }
            return res;
        }
    }
}
