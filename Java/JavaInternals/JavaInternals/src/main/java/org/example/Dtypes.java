package org.example;

import java.math.BigInteger;

public class Dtypes {
    // scalar:
    // primitive integer: byte, short, int, long            |   wrapper Integer: Byte, Short, Integer, Long
    // primitive float: float, double                       |   wrapper float: Float, Double
    //

    // Notes:
    // 1. when an int overflows, it wraps around to negative.(regardless of primitive or non-primitive
    // 2. whenever a datatype has a primitive equivalent, it is converted to that and then the arithmetic operation is performed.

    public static void main(String[] args) {
        int a = 1_200_000;
        int b = 2_000;

        int c = a * b;
        System.out.println("multiplying the 32bit integers: "  + c);        // 609 468 416

        Long aa = 1_200_000L;
        Long bb = 18_000_000L;
        Long cc = aa * bb;
        System.out.println("multiplying 64bit integers: " + cc);

        BigInteger aaa = BigInteger.valueOf(1_200_000);
        BigInteger bbb = BigInteger.valueOf(18_000_000);
        BigInteger ccc = aaa.multiply(bbb);

        System.out.println("multiplying unlimited precision number: " + ccc);
    }
}
