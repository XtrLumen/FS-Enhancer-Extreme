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

import co.nstant.in.cbor.CborException;
import co.nstant.in.cbor.model.DataItem;
import co.nstant.in.cbor.model.Map;
import co.nstant.in.cbor.model.Number;

import org.bouncycastle.asn1.ASN1Encodable;

import java.security.cert.CertificateParsingException;
import java.security.cert.X509Certificate;
import java.util.List;

public class EatAttestation extends Attestation {
    final RootOfTrust rootOfTrust;

    public EatAttestation(X509Certificate x509Cert) throws CertificateParsingException, CborException {
        super(x509Cert);
        Map extension = getEatExtension(x509Cert);

        RootOfTrust.Builder rootOfTrustBuilder = new RootOfTrust.Builder();
        List<Boolean> bootState = null;
        boolean officialBuild = false;

        for (DataItem key : extension.getKeys()) {
            int keyInt = ((Number) key).getValue().intValue();
            switch (keyInt) {
                case EatClaim.VERIFIED_BOOT_KEY:
                    rootOfTrustBuilder.setVerifiedBootKey(CborUtils.getBytes(extension, key));
                    break;
                case EatClaim.DEVICE_LOCKED:
                    rootOfTrustBuilder.setDeviceLocked(CborUtils.getBoolean(extension, key));
                    break;
                case EatClaim.BOOT_STATE:
                    bootState = CborUtils.getBooleanList(extension, key);
                    break;
                case EatClaim.OFFICIAL_BUILD:
                    officialBuild = CborUtils.getBoolean(extension, key);
                    break;
                case EatClaim.VERIFIED_BOOT_HASH:
                    rootOfTrustBuilder.setVerifiedBootHash(CborUtils.getBytes(extension, key));
                    break;
                default:
                    break;
            }
        }

        if (bootState != null) {
            rootOfTrustBuilder.setVerifiedBootState(eatBootStateTypeToVerifiedBootState(bootState, officialBuild));
        }
        rootOfTrust = rootOfTrustBuilder.build();
    }

    public RootOfTrust getRootOfTrust() {
        return rootOfTrust;
    }

    Map getEatExtension(X509Certificate x509Cert) throws CertificateParsingException, CborException {
        byte[] attestationExtensionBytes = x509Cert.getExtensionValue(Attestation.EAT_OID);
        if (attestationExtensionBytes == null || attestationExtensionBytes.length == 0) {
            throw new CertificateParsingException("Did not find extension with OID " + EAT_OID);
        }
        ASN1Encodable asn1 = Asn1Utils.getAsn1EncodableFromBytes(attestationExtensionBytes);
        byte[] cborBytes = Asn1Utils.getByteArrayFromAsn1(asn1);
        return (Map) CborUtils.decodeCbor(cborBytes);
    }

    static int eatBootStateTypeToVerifiedBootState(List<Boolean> bootState, Boolean officialBuild) {
        if (bootState.size() != 5) {
            throw new RuntimeException("Boot state map has unexpected size: " + bootState.size());
        }
        if (bootState.get(4)) {
            throw new RuntimeException("debug-permanent-disable must never be true: " + bootState);
        }
        boolean verifiedOrSelfSigned = bootState.get(0);
        if (verifiedOrSelfSigned != bootState.get(1) && verifiedOrSelfSigned != bootState.get(2) && verifiedOrSelfSigned != bootState.get(3)) {
            throw new RuntimeException("Unexpected boot state: " + bootState);
        }

        if (officialBuild) {
            if (!verifiedOrSelfSigned) {
                throw new AssertionError("Non-verified official build");
            }
            return RootOfTrust.KM_VERIFIED_BOOT_VERIFIED;
        } else {
            return verifiedOrSelfSigned ? RootOfTrust.KM_VERIFIED_BOOT_SELF_SIGNED : RootOfTrust.KM_VERIFIED_BOOT_UNVERIFIED;
        }
    }
}
