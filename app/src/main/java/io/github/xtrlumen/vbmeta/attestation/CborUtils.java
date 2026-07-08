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

import co.nstant.in.cbor.CborDecoder;
import co.nstant.in.cbor.CborException;
import co.nstant.in.cbor.model.Array;
import co.nstant.in.cbor.model.ByteString;
import co.nstant.in.cbor.model.DataItem;
import co.nstant.in.cbor.model.Map;
import co.nstant.in.cbor.model.SimpleValue;
import co.nstant.in.cbor.model.SimpleValueType;

import java.util.ArrayList;
import java.util.List;

class CborUtils {
    public static Boolean getBoolean(Map map, DataItem index) {
        SimpleValueType value = ((SimpleValue) map.get(index)).getSimpleValueType();
        if (value != SimpleValueType.TRUE && value != SimpleValueType.FALSE) {
            throw new RuntimeException("Only expecting boolean values for " + index);
        }
        return (value == SimpleValueType.TRUE);
    }

    public static List<Boolean> getBooleanList(Map map, DataItem index) {
        Array array = (Array) map.get(index);
        List<Boolean> result = new ArrayList<>();
        for (DataItem item : array.getDataItems()) {
            SimpleValueType value = ((SimpleValue) item).getSimpleValueType();
            if (value == SimpleValueType.FALSE) {
                result.add(false);
            } else if (value == SimpleValueType.TRUE) {
                result.add(true);
            } else {
                throw new RuntimeException("Map contains more than booleans: " + map);
            }
        }
        return result;
    }

    public static byte[] getBytes(Map map, DataItem index) {
        DataItem item = map.get(index);
        return ((ByteString) item).getBytes();
    }

    public static DataItem decodeCbor(byte[] encodedBytes) throws CborException {
        return CborDecoder.decode(encodedBytes).get(0);
    }
}
