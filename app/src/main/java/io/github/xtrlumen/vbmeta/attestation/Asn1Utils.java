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

import io.github.xtrlumen.vbmeta.log;

import org.bouncycastle.asn1.ASN1Boolean;
import org.bouncycastle.asn1.ASN1Integer;
import org.bouncycastle.asn1.ASN1Sequence;
import org.bouncycastle.asn1.ASN1Encodable;
import org.bouncycastle.asn1.ASN1Primitive;
import org.bouncycastle.asn1.ASN1Enumerated;
import org.bouncycastle.asn1.DEROctetString;
import org.bouncycastle.asn1.ASN1OctetString;
import org.bouncycastle.asn1.ASN1InputStream;

import java.io.IOException;

import java.math.BigInteger;

import java.security.cert.CertificateParsingException;

public class Asn1Utils {
    public static int getIntegerFromAsn1(ASN1Encodable asn1Value) throws CertificateParsingException {
        if (asn1Value instanceof ASN1Integer) {
            return bigIntegerToInt(((ASN1Integer) asn1Value).getValue());
        } else if (asn1Value instanceof ASN1Enumerated) {
            return bigIntegerToInt(((ASN1Enumerated) asn1Value).getValue());
        } else {
            log.E("Integer value expected, " + asn1Value.getClass().getName() + " found.");
            throw new CertificateParsingException();
        }
    }

    public static byte[] getByteArrayFromAsn1(ASN1Encodable asn1Encodable) throws CertificateParsingException {
        if (!(asn1Encodable instanceof DEROctetString derOctectString)) {
            log.E("Expected DEROctetString");
            throw new CertificateParsingException();
        }
        return derOctectString.getOctets();
    }

    public static ASN1Encodable getAsn1EncodableFromBytes(byte[] bytes) throws CertificateParsingException {
        try (ASN1InputStream asn1InputStream = new ASN1InputStream(bytes)) {
            return asn1InputStream.readObject();
        } catch (IOException e) {
            log.E("Failed to parse Encodable", e);
            throw new CertificateParsingException();
        }
    }

    public static ASN1Sequence getAsn1SequenceFromBytes(byte[] bytes) throws CertificateParsingException {
        try (ASN1InputStream asn1InputStream = new ASN1InputStream(bytes)) {
            return getAsn1SequenceFromStream(asn1InputStream);
        } catch (IOException e) {
            log.E("Failed to parse SEQUENCE", e);
            throw new CertificateParsingException();
        }
    }

    public static ASN1Sequence getAsn1SequenceFromStream(final ASN1InputStream asn1InputStream) throws IOException, CertificateParsingException {
        ASN1Primitive asn1Primitive = asn1InputStream.readObject();
        if (!(asn1Primitive instanceof ASN1OctetString)) {
            log.E("Expected octet stream, found " + asn1Primitive.getClass().getName());
            throw new CertificateParsingException();
        }
        try (ASN1InputStream seqInputStream = new ASN1InputStream(((ASN1OctetString) asn1Primitive).getOctets())) {
            asn1Primitive = seqInputStream.readObject();
            if (!(asn1Primitive instanceof ASN1Sequence)) {
                log.E("Expected sequence, found " + asn1Primitive.getClass().getName());
                throw new CertificateParsingException();
            }
            return (ASN1Sequence) asn1Primitive;
        }
    }

    public static boolean getBooleanFromAsn1(ASN1Encodable value) throws CertificateParsingException {
        if (!(value instanceof ASN1Boolean booleanValue)) {
            log.E("Expected boolean, found " + value.getClass().getName());
            throw new CertificateParsingException();
        }
        if (booleanValue.equals(ASN1Boolean.TRUE)) {
            return true;
        } else if (booleanValue.equals((ASN1Boolean.FALSE))) {
            return false;
        }

        log.E("DER-encoded boolean values must contain either 0x00 or 0xFF");
        throw new CertificateParsingException();
    }

    private static int bigIntegerToInt(BigInteger bigInt) throws CertificateParsingException {
        if (bigInt.compareTo(BigInteger.valueOf(Integer.MAX_VALUE)) > 0 || bigInt.compareTo(BigInteger.ZERO) < 0) {
            log.E("INTEGER out of bounds");
            throw new CertificateParsingException();
        }
        return bigInt.intValue();
    }
}
