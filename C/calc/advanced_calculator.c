#include <stdio.h>

int main() {
    double num1, num2, result;
    char operator;

    printf("Advanced Calculator!\n");

    // This still failed to solve the space passing in the scanf() function.
    // scanf always reads input until it encounters a whitespace character (space, tab, or newline).

    // Input numbers and operator
    printf("Enter first number: ");
    if (scanf("%lf", &num1) != 1) {
        printf("Invalid input for number 1!\n");
        return 1;
    }

    printf("Enter operator (+, -, *, /): ");
    if (scanf(" %c", &operator) != 1) {
        printf("Invalid input for operator!\n");
        return 1;
    }

    printf("Enter second number: ");
    if (scanf("%lf", &num2) != 1) {
        printf("Invalid input for number 2!\n");
        return 1;
    }

    // Perform calculation
    switch (operator) {
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
            if (num2 != 0)
                result = num1 / num2;
            else {
                printf("Error! Division by zero.\n");
                return 1;
            }
            break;
        default:
            printf("Invalid operator!\n");
            return 1;
    }

    // Display result
    printf("Result: %.2lf %c %.2lf = %.2lf\n", num1, operator, num2, result);

    return 0;
}
