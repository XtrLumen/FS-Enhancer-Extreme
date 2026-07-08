package io.github.xtrlumen.vbmeta;

import android.content.ContentProvider;
import android.content.ContentValues;
import android.database.Cursor;
import android.net.Uri;
import android.os.Bundle;
import android.security.keystore.KeyGenParameterSpec;
import android.security.keystore.KeyProperties;

import com.google.common.io.BaseEncoding;

import java.security.KeyPairGenerator;
import java.security.KeyStore;
import java.security.cert.Certificate;
import java.security.cert.X509Certificate;
import java.security.spec.ECGenParameterSpec;

import io.github.xtrlumen.vbmeta.attestation.Attestation;
import io.github.xtrlumen.vbmeta.attestation.RootOfTrust;

public class Provider extends ContentProvider {
    private static final String METHOD_GET = "GET";
    private static final String KEY_ALIAS = "vbmeta_root_of_trust";
    private static final String DEFAULT_CHALLENGE = "vbmeta";

    @Override
    public boolean onCreate() {
        return true;
    }

    @Override
    public Bundle call(String method, String arg, Bundle extras) {
        Bundle result = new Bundle();
        if (!METHOD_GET.equalsIgnoreCase(method)) {
            result.putString("error", "Unsupported method: " + method);
            return result;
        }
        try {
            String challenge = pickString(arg, extras, "challenge", DEFAULT_CHALLENGE);
            RootOfTrust rootOfTrust = loadRootOfTrust(challenge);
            if (rootOfTrust == null) {
                result.putString("error", "No RootOfTrust present in attestation certificate");
                return result;
            }
            String field = extras != null ? extras.getString("field") : null;
            if (field == null || field.isEmpty() || "all".equalsIgnoreCase(field)) {
                putAll(result, rootOfTrust);
            } else if (!putField(result, rootOfTrust, field)) {
                result.putString("error", "Unknown field: " + field);
            }
        } catch (Exception e) {
            result.putString("error", e.getClass().getName() + ": " + e.getMessage());
        }
        return result;
    }

    private static String pickString(String arg, Bundle extras, String key, String fallback) {
        if (arg != null && !arg.isEmpty()) {
            return arg;
        }
        if (extras != null) {
            String value = extras.getString(key);
            if (value != null && !value.isEmpty()) {
                return value;
            }
        }
        return fallback;
    }

    private static void putAll(Bundle result, RootOfTrust rootOfTrust) {
        result.putString("rootOfTrust", rootOfTrust.toString());
        putField(result, rootOfTrust, "deviceLocked");
        putField(result, rootOfTrust, "verifiedBootState");
        putField(result, rootOfTrust, "verifiedBootStateName");
        putField(result, rootOfTrust, "verifiedBootKey");
        putField(result, rootOfTrust, "verifiedBootHash");
    }

    private static boolean putField(Bundle result, RootOfTrust rootOfTrust, String field) {
        switch (field) {
            case "rootOfTrust":
                result.putString("rootOfTrust", rootOfTrust.toString());
                return true;
            case "deviceLocked":
                result.putBoolean("deviceLocked", rootOfTrust.isDeviceLocked());
                return true;
            case "verifiedBootState":
                result.putInt("verifiedBootState", rootOfTrust.getVerifiedBootState());
                return true;
            case "verifiedBootStateName":
                result.putString("verifiedBootStateName",
                        RootOfTrust.verifiedBootStateToString(rootOfTrust.getVerifiedBootState()));
                return true;
            case "verifiedBootKey":
                result.putString("verifiedBootKey", encode(rootOfTrust.getVerifiedBootKey()));
                return true;
            case "verifiedBootHash":
                result.putString("verifiedBootHash", encode(rootOfTrust.getVerifiedBootHash()));
                return true;
            default:
                return false;
        }
    }

    private static String encode(byte[] bytes) {
        return bytes == null ? null : BaseEncoding.base16().lowerCase().encode(bytes);
    }

    private RootOfTrust loadRootOfTrust(String challenge) throws Exception {
        KeyStore keyStore = KeyStore.getInstance("AndroidKeyStore");
        keyStore.load(null);
        if (keyStore.containsAlias(KEY_ALIAS)) {
            keyStore.deleteEntry(KEY_ALIAS);
        }

        KeyPairGenerator generator = KeyPairGenerator.getInstance(
                KeyProperties.KEY_ALGORITHM_EC, "AndroidKeyStore");
        KeyGenParameterSpec spec = new KeyGenParameterSpec.Builder(
                KEY_ALIAS, KeyProperties.PURPOSE_SIGN)
                .setAlgorithmParameterSpec(new ECGenParameterSpec("secp256r1"))
                .setDigests(KeyProperties.DIGEST_SHA256)
                .setAttestationChallenge(challenge.getBytes())
                .build();
        generator.initialize(spec);
        generator.generateKeyPair();

        Certificate[] chain = keyStore.getCertificateChain(KEY_ALIAS);
        if (chain == null || chain.length == 0) {
            throw new IllegalStateException("Empty attestation certificate chain");
        }
        return Attestation.loadFromCertificate((X509Certificate) chain[0]).getRootOfTrust();
    }

    @Override
    public Cursor query(Uri uri, String[] projection, String selection,
                        String[] selectionArgs, String sortOrder) {
        return null;
    }

    @Override
    public String getType(Uri uri) {
        return null;
    }

    @Override
    public Uri insert(Uri uri, ContentValues values) {
        return null;
    }

    @Override
    public int delete(Uri uri, String selection, String[] selectionArgs) {
        return 0;
    }

    @Override
    public int update(Uri uri, ContentValues values, String selection, String[] selectionArgs) {
        return 0;
    }
}
