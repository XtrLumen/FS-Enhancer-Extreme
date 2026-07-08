/*
 * Copyright (C) 2020 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package io.github.xtrlumen.vbmeta.attestation;

import org.bouncycastle.asn1.ASN1Sequence;

import java.security.cert.CertificateParsingException;
import java.security.cert.X509Certificate;

public class Asn1Attestation extends Attestation {
    static final int SW_ENFORCED_INDEX = 6;
    static final int TEE_ENFORCED_INDEX = 7;

    /**
     * Constructs an {@code Asn1Attestation} object from the provided {@link X509Certificate},
     * extracting the attestation data from the attestation extension.
     *
     * @throws CertificateParsingException if the certificate does not contain a properly-formatted
     *     attestation extension.
     */

    public Asn1Attestation(X509Certificate x509Cert) throws CertificateParsingException {
        super(x509Cert);
        ASN1Sequence seq = getAttestationSequence(x509Cert);

        softwareEnforced = new AuthorizationList(seq.getObjectAt(SW_ENFORCED_INDEX));
        teeEnforced = new AuthorizationList(seq.getObjectAt(TEE_ENFORCED_INDEX));
    }

    ASN1Sequence getAttestationSequence(X509Certificate x509Cert) throws CertificateParsingException {
        byte[] attestationExtensionBytes = x509Cert.getExtensionValue(Attestation.ASN1_OID);
        if (attestationExtensionBytes == null || attestationExtensionBytes.length == 0) {
            throw new CertificateParsingException("Did not find extension with OID " + ASN1_OID);
        }
        return Asn1Utils.getAsn1SequenceFromBytes(attestationExtensionBytes);
    }

    public RootOfTrust getRootOfTrust() {
        RootOfTrust tee = teeEnforced.getRootOfTrust();
        if (tee != null) return tee;
        return softwareEnforced.getRootOfTrust();
    }
}
