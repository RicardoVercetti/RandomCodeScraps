#include <stdio.h>
#include <string.h>

/// syntax 
// struct struct_name {
//     field_type field_name;
//     field_type field_name;
//     field_type field_name;
// }

// with struct definitions
// struct structure_name
// {
//     field_type field_name;
//     field_type field_name;
//     field_type field_name;
//     // ...
// } struct_var1, struct_var2;

// without struct definitions
// struct
// {
//     field_type field_name;
//     field_type field_name;
//     field_type field_name;
//     // ...
// } struct_var1, struct_var2;

int main() {
    puts("defining struct in C...");

    struct address {
        unsigned int house_number;
        char street[100];
        char city[50];
        char state[4];      // 3 + '\0'
        int zip_code;
        char country[50];
    } 
    // home_address, business_address
    ;

    // or define the variables separately
    // struct address business_address, home_address;

    struct address business_address = {
        200,
        "Venisula Street",
        "Penson state",
        "Ari",
        2456643,
        "Canada"
    };


    printf("address: %d, %s, %s\n", business_address.house_number, business_address.street, business_address.city);
    printf("state: %s\n", business_address.state);
    printf("country: %s - %d\n", business_address.country, business_address.zip_code);


    // one struct may contain other structs
    struct customer {
        char name[50];
        struct address shipping_address;
        struct address billing_address;
    } john;

    // john.name = {0};
    strcpy(john.name, "A");
    // john.name[0] = 'A';
    // john.name[1] = '\0';

    strcpy(john.shipping_address.city, "San Francisco");

    printf("addres of name: %p\n", &john.name);
    printf("customer name: %s\n", john.name);
    printf("binary name: %p\n", *john.name);
    printf("shipping addr: %s\n", john.shipping_address.city);

    printf("sizeof name: %d\n", sizeof(john.name));

    return 0;
}