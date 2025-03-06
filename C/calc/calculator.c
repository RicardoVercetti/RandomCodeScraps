#include <stdio.h>

int main() {
    double num1, num2, result;
    char operator;

    // Explaination for %lf
    // l (long) → Used for long floating-point numbers.
    // f (float) → Represents a floating-point number.

    // In printf(), you can use "%f" for both float and double because printf() automatically promotes float to double.
    // In scanf(), %f is for float, and %lf is required for double.

    // Explaination for Wierd behaviour 
    //      - if more charecters are passed in to the scanf()
    //      - if white space is passed in to the scanf()

    // scanf() reads input until it encounters a whitespace character (space, tab, or newline).


    printf("Enter first number: ");
    scanf("%lf", &num1);                // lf is required for double()
    printf("Enter operator: ");
    scanf(" %c", &operator);
    printf("Enter second number: ");
    scanf("%lf", &num2);

    // Perform calculation
    switch (operator)
    {
    case '+':
        result = num1 + num2;
        break;
    case '-':
        result = num1 - num2;
        break;
    case '*':
        result = num1 * num2;
        break;
    case '/':
        result = num1 / num2;
        break;
    default:
        printf("Invalid operator!\n");
        return 1;
    }

    // Display result
    printf("Result: %.2lf %c %.2lf = %.2lf\n", num1, operator, num2 ,result);

    return 0;
}