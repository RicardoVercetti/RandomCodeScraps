#include <stdio.h>
#include <stdlib.h>

int main() {
    double num1, num2, result;
    char operator;
    char input[100]; // Buffer for input

    // Input first number
    printf("Enter first number: ");
    fgets(input, sizeof(input), stdin);  // Read the whole line
    if (sscanf(input, "%lf", &num1) != 1) {
        printf("Invalid input for number 1!\n");
        return 1;
    }

    // Input operator
    printf("Enter operator (+, -, *, /): ");
    fgets(input, sizeof(input), stdin);
    if (sscanf(input, " %c", &operator) != 1) {
        printf("Invalid input for operator!\n");
        return 1;
    }

    // Input second number
    printf("Enter second number: ");
    fgets(input, sizeof(input), stdin);
    if (sscanf(input, "%lf", &num2) != 1) {
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
