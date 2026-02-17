import java.util.Base64;
import java.util.HexFormat;

import javax.crypto.Cipher;
import javax.crypto.KeyGenerator;
import javax.crypto.SecretKey;

public class EncryptionMethod {

    static String KEY_ALGORITHM = "AES";
    static String ALGORITHM = "AES/GCM/NoPadding";            // using `AES` defaults to `ECB` mode

    // Generate AES Key (128-bit)
    public static SecretKey generateKey() throws Exception {
        KeyGenerator keyGenerator = KeyGenerator.getInstance(KEY_ALGORITHM);
        keyGenerator.init(128); // 128, 192, or 256
        return keyGenerator.generateKey();
    }

    // Encrypt
    public static String encrypt(String data, SecretKey key) throws Exception {
        Cipher cipher = Cipher.getInstance(ALGORITHM);
        cipher.init(Cipher.ENCRYPT_MODE, key);
        byte[] encryptedBytes = cipher.doFinal(data.getBytes());
        return Base64.getEncoder().encodeToString(encryptedBytes); // whats this encoder?
    }

    // Decrypt
    public static String decrypt(String encryptedData, SecretKey key) throws Exception {
        Cipher cipher = Cipher.getInstance(ALGORITHM);
        cipher.init(Cipher.DECRYPT_MODE, key);
        byte[] decodedBytes = Base64.getDecoder().decode(encryptedData);
        byte[] decryptedBytes = cipher.doFinal(decodedBytes);
        return new String(decryptedBytes);
    }

    public static void main(String[] args) throws Exception {
        System.out.println("this runs...");

        String input = "Hi galaxy";
        System.out.println("Input: '" + input + "'");
        SecretKey key = generateKey();              // 5e19d34c2ee78046726c404e854de23a
        System.out.println("key: " + toHex(key.getEncoded()));

        String encryptedString = encrypt(input, key);
        System.out.println("encrypted value: '" + encryptedString + "'");


        String decryptecdString = decrypt(encryptedString, key);
        System.out.println("decrypted value: '" + decryptecdString + "'");
    }

    static String toHex(byte[] bytes) {
        return HexFormat.of().formatHex(bytes);
    }
}
