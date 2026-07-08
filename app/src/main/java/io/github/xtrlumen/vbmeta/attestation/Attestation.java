/*
 * Copyright (C) 2016 The Android Open Source Project
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

import com.google.common.collect.ImmutableSet;

import java.security.cert.CertificateParsingException;
import java.security.cert.X509Certificate;
import java.util.Set;

import co.nstant.in.cbor.CborException;

/**
 * Parses an attestation certificate and provides an easy-to-use interface for examining the
 * contents.
 */
public abstract class Attestation {
    static final String EAT_OID = "1.3.6.1.4.1.11129.2.1.25";
    static final String ASN1_OID = "1.3.6.1.4.1.11129.2.1.17";
    static final String KEY_USAGE_OID = "2.5.29.15"; // Standard key usage extension.

    public static final int KM_SECURITY_LEVEL_SOFTWARE = 0;
    public static final int KM_SECURITY_LEVEL_TRUSTED_ENVIRONMENT = 1;
    public static final int KM_SECURITY_LEVEL_STRONG_BOX = 2;

    int attestationVersion;
    int keymasterVersion;
    int keymasterSecurityLevel;
    byte[] attestationChallenge;
    byte[] uniqueId;
    AuthorizationList softwareEnforced;
    AuthorizationList teeEnforced;
    Set<String> unexpectedExtensionOids;

    /**
     * Constructs an {@code Attestation} object from the provided {@link X509Certificate},
     * extracting the attestation data from the attestation extension.
     *
     * <p>This method ensures that at most one attestation extension is included in the certificate.
     *
     * @throws CertificateParsingException if the certificate does not contain a properly-formatted
     *                                     attestation extension, if it contains multiple attestation extensions, or if the
     *                                     attestation extension can not be parsed.
     */

    public static Attestation loadFromCertificate(X509Certificate x509Cert) throws CertificateParsingException {
        if (x509Cert.getExtensionValue(EAT_OID) == null
                && x509Cert.getExtensionValue(ASN1_OID) == null) {
            throw new CertificateParsingException("No attestation extensions found");
        }
        if (x509Cert.getExtensionValue(EAT_OID) != null) {
            if (x509Cert.getExtensionValue(ASN1_OID) != null) {
                throw new CertificateParsingException("Multiple attestation extensions found");
            }
            try {
                return new EatAttestation(x509Cert);
            } catch (CborException cbe) {
                throw new CertificateParsingException("Unable to parse EAT extension", cbe);
            }
        }
        return new Asn1Attestation(x509Cert);
    }

    Attestation(X509Certificate x509Cert) {
        unexpectedExtensionOids = retrieveUnexpectedExtensionOids(x509Cert);
    }

    public abstract RootOfTrust getRootOfTrust();

    Set<String> retrieveUnexpectedExtensionOids(X509Certificate x509Cert) {
        return new ImmutableSet.Builder<String>()
                .addAll(x509Cert.getCriticalExtensionOIDs()
                        .stream()
                        .filter(s -> !KEY_USAGE_OID.equals(s))
                        .iterator())
                .addAll(x509Cert.getNonCriticalExtensionOIDs()
                        .stream()
                        .filter(s -> !ASN1_OID.equals(s) && !EAT_OID.equals(s))
                        .iterator())
                .build();
    }
}
