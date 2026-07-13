// infix to prefix and postfix
    // B O D M A S
    // a + b * c            => (a + (b * c))                    => +a*bc
    // a * (b + c) / e + f  => (((a * (b + c)) / e) + f)        => +/*a+bcef
    // (A + B) * C - (D + E) / (F + G) => ((A + B) * C) - ((D + E)/(F + G)) => -*+ABC/+DE+FG
