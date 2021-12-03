// These can for example be executed from Node.JS REPL with the input being in the same directory as "input.txt"
// If ran from a file instead, wrap inside console.log() and replace fs with require("fs")
// You could also replace "./input.txt" with 0 and then after properly escaping all quotes
// you would be able to do `node -e '...' < input`

// Day 1 Part 1
fs.readFileSync("./input.txt", "utf-8").trim().split("\n").map(Number).reduce((t, v, i, a) => t + ((a[i - 1] ?? 0) < v), 0) - 1;
// Day 1 Part 2
fs.readFileSync("./input.txt", "utf-8").trim().split("\n").map(Number).reduce(([p, t], v, i, a) => i < a.length - 2 ? [v+a[i+1]+a[i+2], t + (v+a[i+1]+a[i+2] > p)] : [p, t], [1e5, 0])[1];
// Day 2 Part 1
fs.readFileSync("./input.txt", "utf-8").trim().split("\n").map((l) => l.split(" ").map(e => isNaN(parseInt(e)) ? e : parseInt(e))).reduce(([x, y], [dir, amount]) => [dir === "forward" ? x + amount : x, dir === "down" ? y + amount : (dir === "up" ? y - amount : y)], [0, 0]).reduce((t, a) => t * a, 1);
// Day 2 Part 2
fs.readFileSync("./input.txt", "utf-8").trim().split("\n").map((l) => l.split(" ").map(e => isNaN(parseInt(e)) ? e : parseInt(e))).reduce(([x, y, aim], [dir, amount]) => [dir === "forward" ? x + amount : x, dir === "forward" ? y + aim * amount : y, dir === "up" ? aim - amount : (dir === "down" ? aim + amount : aim)], [0, 0, 0]).slice(0,2).reduce((t, a) => t * a, 1);
// Day 3 Part 1
[0, 0].fill(fs.readFileSync("./input.txt", "utf-8").trim().split("\n").reduce((t, v) => [...Array(12)].map((_, i) => t[i] + (v[i] === "1")), [...Array(12)].fill(0)).map(e => e > 500 ? "1" : "0").join("")).map((e, i) => i ? e.replace(/[01]/g, c => ({"1": "0", "0":"1"})[c]) : e).map(e => parseInt(e, 2)).reduce((t, e) => t * e, 1);
// Day 3 Part 2
[...Array(12)].reduce(([ox, co], _, i) => [ox.length > 1 ? ox.filter(l => l[i] === (ox.reduce((t, l2) => t + (l2[i] === "1"), 0) >= Math.ceil(ox.length / 2) ? "1" : "0")) : ox, co.length > 1 ? co.filter(l => l[i] === (co.reduce((t, l2) => t + (l2[i] === "1"), 0) >= Math.ceil(co.length / 2) ? "0" : "1")) : co], [0, 0].fill(fs.readFileSync("./input.txt", "utf-8").trim().split("\n").map(l => l.trim())).map(r => [...r])).map(e => parseInt(e[0], 2)).reduce((t, e) => t * e, 1);
