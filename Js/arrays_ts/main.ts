// things to do in arrays

const minimalArrays = () => {
    let fruit: string[] = ["apple", "banana", "mango"]; // minimal array
    let nums: Array<number> = [1,2,3,4,5];
    console.log(fruit);
    console.log(nums);
}

const emptyArray = () => {
    let items: string[] = [];   // empty arrays
    items.push("pen");
    items.push("notebook");

    console.log(items);
}

const fixedLengthTypedArrays = () => {
    let person: [string, number] = ["Alice", 25];
    console.log(person);
}

const readonlyArrays = () => {
    let colors: readonly string[] = ["red", "green", "blue"];
    // colors.push("yellow");
    console.log(colors);
}

const arrayOfObject = () => {
    type Person = {
        namai: string,
        lastNamai: string,
        age: number,
    }

    let persons: Person[] = [];

    persons.push({ namai: "alexis", lastNamai: "john", age: 34 });
    persons.push({ namai: "pineapple", lastNamai: "luffy", age: 21 });
    persons.push({ namai: "sakura", lastNamai: "haruno", age: 27 });
    persons.push({ namai: "ichigo", lastNamai: "kurosaki", age: 29 });
    persons.push({ namai: "light", lastNamai: "yagami", age: 24 });
    persons.push({ namai: "mikasa", lastNamai: "ackerman", age: 26 });
    persons.push({ namai: "gintoki", lastNamai: "sakata", age: 35 });
    persons.push({ namai: "hinata", lastNamai: "hyuga", age: 25 });
    persons.push({ namai: "eren", lastNamai: "yeager", age: 23 });
    persons.push({ namai: "lelouch", lastNamai: "lamperouge", age: 28 });

    // console.log(persons);
    const firstPerson = persons[persons.length - 1];
    

    console.log(firstPerson?.age);

}

const byItem = () => {
    type Person = {
        namai: string,
        lastNamai: string,
        age: number,
    }

    let persons: Person[] = [];

    persons.push({ namai: "alexis", lastNamai: "john", age: 34 });
    persons.push({ namai: "pineapple", lastNamai: "luffy", age: 21 });
    persons.push({ namai: "sakura", lastNamai: "haruno", age: 27 });
    persons.push({ namai: "ichigo", lastNamai: "kurosaki", age: 29 });
    persons.push({ namai: "light", lastNamai: "yagami", age: 24 });
    persons.push({ namai: "mikasa", lastNamai: "ackerman", age: 26 });
    persons.push({ namai: "gintoki", lastNamai: "sakata", age: 35 });
    persons.push({ namai: "hinata", lastNamai: "hyuga", age: 25 });
    persons.push({ namai: "eren", lastNamai: "yeager", age: 23 });
    persons.push({ namai: "lelouch", lastNamai: "lamperouge", age: 28 });

    for (const person of persons) {
        if (person.namai === "mikasa") {
            console.log(person);
        }
    }

    console.log("finished looping...");
}

// map
const mapTheArray = () => {
    let nums = [1, 2, 3, 4, 5, 6];

    // let newNums = nums.map(num => num * num);

    // let newNums = nums.map((val, ind, num) => {
    //     val = val+1;
    //     return val;
    // });

    let newNums = nums.filter;
    console.log(typeof newNums);


    console.log(newNums);
    console.log(nums);
}




// filter, last, map, slice, by item, 

//minimalArrays();
// emptyArray();
// fixedLengthTypedArrays();
// readonlyArrays();
// arrayOfObject();
// byItem();
mapTheArray();