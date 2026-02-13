import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.util.HexFormat;

public class HashingIntro {
    // Hashing Algorithmn

    // broken/depricated
    // 1. MD5
    // 2. SHA-1

    // Legacy
    // SHA-256(raw)
    // SHA-512(raw)

    // Secure(context-dependent)
    // SHA-256
    // SHA-512
    // SHA-3
    
    // Password hashing
    // 1. PBKDF2 (Standard Java)
    // 2. bcrypt/scrypt/Argon2
    
    public static void main(String[] args) throws Exception {
        System.out.println("Some intro of hashing...");

        MessageDigest digest = MessageDigest.getInstance("SHA-256");
        byte[] input = "hello world".getBytes(StandardCharsets.UTF_8);      // b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
        byte[] hash = digest.digest(input);

        byte[] secondInput = "hello world".getBytes(StandardCharsets.UTF_16);            // 593ee3b5b58cae24f7f38c0c90f1115bf2268ee08aaf48cb5328d595dc1c4f83
        byte[] secondHash = digest.digest(secondInput);

        System.out.println("byte value: " + toHex(input));                      // 68656c6c6f20776f726c64
        System.out.println("hash value: " + toHex(hash));
        System.out.println("second byte value: " + toHex(secondInput));         // feff 0068 0065 006c 006c 006f 0020 0077 006f 0072 006c 0064
        System.out.println("second hash value: " + toHex(secondHash));

        
    }

    static String toHex(byte[] bytes) {
        return HexFormat.of().formatHex(bytes);
    }
}
