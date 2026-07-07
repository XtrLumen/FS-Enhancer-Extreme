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

package android.hardware.security.keymint;

import android.hardware.security.keymint.DeviceInfo;
import android.hardware.security.keymint.MacedPublicKey;
import android.hardware.security.keymint.ProtectedData;
import android.hardware.security.keymint.RpcHardwareInfo;

interface IRemotelyProvisionedComponent {
    const int STATUS_FAILED = 1;
    const int STATUS_INVALID_MAC = 2;
    const int STATUS_PRODUCTION_KEY_IN_TEST_REQUEST = 3;
    const int STATUS_TEST_KEY_IN_PRODUCTION_REQUEST = 4;
    const int STATUS_INVALID_EEK = 5;
    const int STATUS_REMOVED = 6;

    RpcHardwareInfo getHardwareInfo();

    byte[] generateEcdsaP256KeyPair(in boolean testMode, out MacedPublicKey macedPublicKey);

    byte[] generateCertificateRequest(in boolean testMode, in MacedPublicKey[] keysToSign,
            in byte[] endpointEncryptionCertChain, in byte[] challenge, out DeviceInfo deviceInfo,
            out ProtectedData protectedData);

    byte[] generateCertificateRequestV2(in MacedPublicKey[] keysToSign, in byte[] challenge);
}
