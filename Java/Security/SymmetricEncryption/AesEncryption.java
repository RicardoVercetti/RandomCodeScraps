import java.nio.charset.StandardCharsets;
import java.security.InvalidAlgorithmParameterException;
import java.security.InvalidKeyException;
import java.security.NoSuchAlgorithmException;
import java.security.SecureRandom;
import java.util.HexFormat;

import javax.crypto.BadPaddingException;
import javax.crypto.Cipher;
import javax.crypto.IllegalBlockSizeException;
import javax.crypto.KeyGenerator;
import javax.crypto.NoSuchPaddingException;
import javax.crypto.SecretKey;
import javax.crypto.spec.GCMParameterSpec;

public class AesEncryption {

    public static void symmetricEncryptionWithAes() throws 
    NoSuchAlgorithmException,                   // at the KeyGenerator.getInstance()
    NoSuchPaddingException,                     // at the Cipher.getInstance()
    InvalidKeyException,                        // at the cipher.init() if key is wrong 
    InvalidAlgorithmParameterException,         // at the cipher.init() if mode is wrong 
    IllegalBlockSizeException,                  // where tf is this from? 
    BadPaddingException                         // where tf is this from? 
    {
        // -- Symmetric encryption --

        // #1: Generate a key
        KeyGenerator keyGen = KeyGenerator.getInstance("AES");
        keyGen.init(256); // AES only supports generation of 3 diffent lengths of keys → 128 bits (16 bytes), 192 bits (24 bytes), 256 bits (32 bytes)
        SecretKey key = keyGen.generateKey();

        System.out.println("key: " + toHex(key.getEncoded()));      // sample: 3de64b5d399893ff7f7c39953cdc015a9cfed9a10203c2e90ae4d4df632963de

        // #2: Generate a nonce(IV)   → WTF is nonce?
        byte[] iv = new byte[12]; // 96 bytes(said to be recommended for GCM) → but why?
        SecureRandom secureRandom = new SecureRandom();
        secureRandom.nextBytes(iv);         // it fills the empty byte array with random bytes?
        // IV must be unique per encryption with the same key → but why?

        System.out.println("iv: " + toHex(iv));     // sample: 51a2f91992c997dd2a6020f7

        // #3: Encrypt
        Cipher cipherEnc = Cipher.getInstance("AES/GCM/NoPadding");
        GCMParameterSpec spec = new GCMParameterSpec(128, iv);      // 128 bit auth tag, the encrypted cipher has encrypted data and the authenticaiton tag, here 128 bits is the length of authenticaion tag
        cipherEnc.init(Cipher.ENCRYPT_MODE, key, spec);

        // System.out.println("spec: " + spec.hashCode());

        byte[] plainText = "Hello 'secured by java' world".getBytes(StandardCharsets.UTF_8);        // whats the difference between just calling .getBytes() and passing in the encoding?
        byte[] cipherText = cipherEnc.doFinal(plainText);         // the cipher text contains → [ encrypted data || authentication tag ]


        // #4: Decrypt
        Cipher cipherDec = Cipher.getInstance("AES/GCM/NoPadding");
        GCMParameterSpec anotherSpec = new GCMParameterSpec(128, iv); // possible values: {128, 120, 112, 104, 96}
        cipherDec.init(Cipher.DECRYPT_MODE, key, anotherSpec);

        byte[] decryptedCipher = cipherDec.doFinal(cipherText);
        String message = new String(decryptedCipher, StandardCharsets.UTF_8);

        System.out.println("decrypted text is: ^" + message + "$");
    }
    public static void main(String[] args) throws Exception {
        System.out.println("AES encryption in a NUTSHELL...");
        symmetricEncryptionWithAes();
    }

    static String toHex(byte[] bytes) {
        return HexFormat.of().formatHex(bytes);
    }
}
