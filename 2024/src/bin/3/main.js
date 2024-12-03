// Read input file
const fs = require("fs");
const path = require("path");
const input = fs.readFileSync(path.join(__dirname, "input.txt"), "utf8");

// Parse input
const re = /mul\((\d+),(\d+)\)/g;
const matches = input.match(re);

const sum = matches.reduce((acc, match) => {
  const [v1, v2] = match.substring(4, match.length - 1).split(",");
  return acc + parseInt(v1) * parseInt(v2);
}, 0);

console.log("number of matches", matches.length);

console.log(sum);

const re2 = /(mul\(\d+,\d+\))|(do\(\))|(don't\(\))/g;
const matches2 = input.match(re2);
console.log("number of new matches", matches2.length);
let sum2 = 0;
let do_count = false;
for (const match of matches2) {
  if (match === "do()") {
    do_count = true;
  } else if (match === "don't()") {
    do_count = false;
  } else if (do_count) {
    const str = match.substring(4, match.length - 1);
    const vals = str.split(",").map((v) => parseInt(v));
    sum2 += vals[0] * vals[1];
  }
}
console.log(sum2);
