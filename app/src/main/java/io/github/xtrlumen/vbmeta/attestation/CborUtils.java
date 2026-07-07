package io.github.xtrlumen.vbmeta.attestation;

import co.nstant.in.cbor.CborDecoder;
import co.nstant.in.cbor.CborException;
import co.nstant.in.cbor.model.Array;
import co.nstant.in.cbor.model.ByteString;
import co.nstant.in.cbor.model.DataItem;
import co.nstant.in.cbor.model.Map;
import co.nstant.in.cbor.model.NegativeInteger;
import co.nstant.in.cbor.model.Number;
import co.nstant.in.cbor.model.SimpleValue;
import co.nstant.in.cbor.model.SimpleValueType;
import co.nstant.in.cbor.model.UnicodeString;
import co.nstant.in.cbor.model.UnsignedInteger;

import java.nio.charset.StandardCharsets;
import java.util.ArrayList;
import java.util.Date;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

class CborUtils {
    public static Number toNumber(long l) {
        return l >= 0 ? new UnsignedInteger(l) : new NegativeInteger(l);
    }

    public static int getInt(Map map, long index) {
        DataItem item = map.get(CborUtils.toNumber(index));
        return ((Number) item).getValue().intValue();
    }

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

    public static String getUnicodeString(Map map, DataItem index) {
        DataItem item = map.get(index);
        return ((UnicodeString) item).getString();
    }

    public static DataItem decodeCbor(byte[] encodedBytes) throws CborException {
        return CborDecoder.decode(encodedBytes).get(0);
    }
}
