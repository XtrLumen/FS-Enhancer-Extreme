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
import co.nstant.in.cbor.model.Number;
import co.nstant.in.cbor.model.SimpleValue;
import co.nstant.in.cbor.model.SimpleValueType;

import java.io.ByteArrayOutputStream;
import java.nio.charset.StandardCharsets;

import java.util.ArrayList;
import java.util.Date;
import java.util.HashSet;
import java.util.List;
import java.util.Set;


class CborUtils {
    public static int getInt(Map map, DataItem index) {
        DataItem item = map.get(index);
        return ((Number) item).getValue().intValue();
    }

    public static long getLong(Map map, DataItem index) {
        DataItem item = map.get(index);
        return ((Number) item).getValue().longValue();
    }

    public static Set<Integer> getIntSet(Map map, DataItem index) {
        Array array = (Array) map.get(index);
        Set<Integer> result = new HashSet<>();
        for (DataItem item : array.getDataItems()) {
            result.add(((Number) item).getValue().intValue());
        }
        return result;
    }

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

    public static Date getDate(Map map, DataItem index) {
        DataItem item = map.get(index);
        long epochMillis = ((Number) item).getValue().longValue();
        return new Date(epochMillis);
    }

    public static byte[] getBytes(Map map, DataItem index) {
        DataItem item = map.get(index);
        return ((ByteString) item).getBytes();
    }

    public static String getString(Map map, DataItem index) {
        byte[] bytes = getBytes(map, index);
        return new String(bytes, StandardCharsets.UTF_8);
    }

    public static DataItem decodeCbor(byte[] encodedBytes) throws CborException {
        return CborDecoder.decode(encodedBytes).get(0);
    }
}
