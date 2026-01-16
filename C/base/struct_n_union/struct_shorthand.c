#include <stdio.h>

typedef struct {
        int id;
        char name[20];
        int age;
        char address[30];
} person;

void view_person(person *p) {
    printf("person name: %s\n", p -> name);
    printf("addr: %p\n", p);
}

int main() {
    puts("trying out typedef...");

    

    person p1 = {
        1,
        "Alan",
        20,
        "20A, Techtown, UK"
    };

    printf("addr out: %p\n", &p1);

    view_person(&p1);

    return 0;
}