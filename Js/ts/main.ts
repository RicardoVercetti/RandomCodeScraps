import fs from "node:fs";

console.log("Main initialized");

fs.readFile("./package.json", (err, data) => {
    setTimeout(() => {
        console.log("timeout...");
    });

    setImmediate(() => {
        console.log("immediate..");
    });

    if (err) {
        console.log(`error reading data: ${err}`);
    } else {
        console.log(`file contents: ${data}`);
    }

    console.log(Object.keys(data));
    console.log(`first: ${data}`);
});


console.log("End");


