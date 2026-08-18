/*
 * This file is part of FS-Enhancer-Extreme.
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
 * without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with this program;
 * If not, see <https://www.gnu.org/licenses/>.
 *
 * Copyright (C) 2025-2026 XtrLumen
 */

package io.github.xtrlumen.vbmeta;

import io.github.xtrlumen.vbmeta.log;

import android.content.ContentValues;
import android.content.ContentProvider;

import android.net.Uri;

import android.os.Bundle;

import android.database.Cursor;

import android.security.keystore.KeyProperties;
import android.security.keystore.KeyGenParameterSpec;

import java.security.KeyStore;
import java.security.KeyPairGenerator;

import java.security.cert.Certificate;
import java.security.cert.X509Certificate;

import java.security.spec.ECGenParameterSpec;

import com.google.common.io.BaseEncoding;

import io.github.xtrlumen.vbmeta.attestation.Attestation;
import io.github.xtrlumen.vbmeta.attestation.RootOfTrust;

public class Entry extends ContentProvider {
    private RootOfTrust loadRootOfTrust() throws Exception {
        String KEY_ALIAS = "root_of_trust";

        KeyStore keyStore = KeyStore.getInstance("AndroidKeyStore");
        keyStore.load(null);
        if (keyStore.containsAlias(KEY_ALIAS)) {
            keyStore.deleteEntry(KEY_ALIAS);
        }

        KeyPairGenerator keyPairGenerator = KeyPairGenerator.getInstance(KeyProperties.KEY_ALGORITHM_EC, "AndroidKeyStore");
        KeyGenParameterSpec keyGenParameterSpec = new KeyGenParameterSpec.Builder(KEY_ALIAS, KeyProperties.PURPOSE_SIGN).setAlgorithmParameterSpec(new ECGenParameterSpec("secp256r1")).setDigests(KeyProperties.DIGEST_SHA256).setAttestationChallenge("stub".getBytes()).build();
        keyPairGenerator.initialize(keyGenParameterSpec);
        keyPairGenerator.generateKeyPair();

        Certificate[] chain = keyStore.getCertificateChain(KEY_ALIAS);
        if (chain == null || chain.length == 0) {
            throw new IllegalStateException("Empty attestation certificate chain");
        }

        return Attestation.loadFromCertificate((X509Certificate) chain[0]).getRootOfTrust();
    }

    @Override
    public Bundle call(String method, String stub, Bundle extra) {
        log.D("用于测试log_raw:多行日志一");
        log.D("用于测试log_raw:多行日志二");

        Bundle result = new Bundle();

        if (!"GET".equals(method)) {
            result.putString("undefined", method);
            return result;
        }

        RootOfTrust rootOfTrust;
        try {
            rootOfTrust = loadRootOfTrust();
        } catch (Exception e) {
            result.putString("error", e.toString());
            return result;
        }
        if (rootOfTrust == null) {
            result.putString("error", "No RootOfTrust present in attestation certificate");
            return result;
        }

        String field = extra.getString("field");
        if (field == null) {
            result.putString("rootOfTrust", "\n" + rootOfTrust + "\n");
        } else if ("verifiedBootKey".equals(field)) {
            result.putString(field, BaseEncoding.base16().lowerCase().encode(rootOfTrust.getVerifiedBootKey()) + "=" + field);
        } else if ("deviceLocked".equals(field)) {
            result.putBoolean(field, rootOfTrust.isDeviceLocked());
        } else if ("verifiedBootState".equals(field)) {
            result.putString(field, RootOfTrust.verifiedBootStateToString(rootOfTrust.getVerifiedBootState()));
        } else if ("verifiedBootHash".equals(field)) {
            result.putString(field, BaseEncoding.base16().lowerCase().encode(rootOfTrust.getVerifiedBootHash()) + "=" + field);
        } else {
            result.putString("undefined", field);
        }

        return result;
    }

    @Override
    public boolean onCreate() {
        return true;
    }
    @Override
    public String getType(Uri stub1) {
        return null;
    }
    @Override
    public Uri insert(Uri stub2, ContentValues stub3) {
        return null;
    }
    @Override
    public int delete(Uri stub4, String stub5, String[] stub6) {
        return 0;
    }
    @Override
    public int update(Uri stub7, ContentValues stub8, String stub9, String[] stub10) {
        return 0;
    }
    @Override
    public Cursor query(Uri stub11, String[] stub12, String stub13, String[] stub14, String stub15) {
        return null;
    }
}
