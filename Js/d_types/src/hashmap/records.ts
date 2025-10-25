// hashmaps in js


// 1. Record 

const map: Record<string, number> = {};
 
map["apple"] = 10;
map["bannana"] = 20;
map["mango"] = 60;

// console.log(map);

// delete map["mango"];

// console.log(map);


for(const key in map) {
    console.log(key, map[key]);
}
