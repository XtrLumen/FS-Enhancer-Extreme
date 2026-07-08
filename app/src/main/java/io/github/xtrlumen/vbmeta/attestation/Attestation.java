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

import java.security.cert.CertificateParsingException;
import java.security.cert.X509Certificate;

import co.nstant.in.cbor.CborException;

public abstract class Attestation {
    static final String EAT_OID = "1.3.6.1.4.1.11129.2.1.25";
    static final String ASN1_OID = "1.3.6.1.4.1.11129.2.1.17";

    AuthorizationList softwareEnforced;
    AuthorizationList teeEnforced;

    public static Attestation loadFromCertificate(X509Certificate x509Cert) throws CertificateParsingException {
        if (x509Cert.getExtensionValue(EAT_OID) == null && x509Cert.getExtensionValue(ASN1_OID) == null) {
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
    }

    public abstract RootOfTrust getRootOfTrust();
}
