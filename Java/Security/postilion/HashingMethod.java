import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.util.HexFormat;

public class HashingMethod {
    public static void main(String[] args) throws Exception {
        System.out.println("runs...");

        // hashing at its finest
        MessageDigest digest = MessageDigest.getInstance("SHA-224");
        String inputString = "input string";
        // Hashing verities
        // MD2      | 16 bytes → 2aeb6b3e43d75847513472c69916d49e
        // MD5      | 16 bytes → 164c375b4a5df44a332ca34bda6cba9d
        // SHA-1    | 20 bytes → b1a39a26ea62a5c075cd3cb5aa46492c8e1134b7
        // SHA-224  | 28 bytes → 9f54a20be492af59bc01253b1303b9a7d6ded12e10e2f3ee86196a4e
        // SHA-256  | 32 bytes → f23f4781d6814ebe349c6b230c1f700714f4f70f735022bd4b1fb69421859993
        // SHA-384  | 48 bytes → 9be3e778129db2f6a4b33345eb88c1f3706a6b5258f842c86888dada1840ea8a64114fe197fb5bc965027ff34ffe7c1e
        // SHA-512  | 64 bytes → 61fdf527eb4a1a793633ea745c36ae06f197b565f07ea0e2254c15064bd8c744d8e66b73c55b409b3dbcb3c3cf4f52d3f234e3dfd7cd4a344bb8d83bbf0094db

        System.out.println("hash: " + toHex(digest.digest(toBytes(inputString))));

    }

    static String toHex(byte[] array) {
        return HexFormat.of().formatHex(array);
    }

    static byte[] toBytes(String str) {
        return str.getBytes(StandardCharsets.UTF_8);
    }
}
