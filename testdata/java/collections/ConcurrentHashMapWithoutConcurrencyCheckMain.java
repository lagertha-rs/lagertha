package collections.concurrent_hash_map_without_concurrency_check;

import java.util.concurrent.ConcurrentHashMap;

public class ConcurrentHashMapWithoutConcurrencyCheckMain {
    public static void main(String[] args) {
        ConcurrentHashMap<String, String> map = new ConcurrentHashMap<>();
        map.put("java.version", "24");
        var version = map.get("java.version");
    }
}