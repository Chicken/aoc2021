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
// Day 4 Part 1
([ns, ...bs] = fs.readFileSync("./input.txt", "utf-8").trim().split("\n\n").map((v, i) => i ? v.split("\n").map(r => r.trim().split(/ +/g).map(Number)) : v.split(",").map(Number)))[0].reduce(([u, w], n) => w ? [u, w] : [u.concat(n), bs.reduce((w2, b) => w2 ? w2 : ((b.map(r => r.reduce((t, c) => (u.includes(c) || c == n) ? t + 1 : t, 0)).some(v => v == 5) || [...Array(5)].map((_, c) => [...Array(5)].reduce((t, _, r) => (u.includes(b[r][c]) || b[r][c] == n) ? t + 1 : t, 0)).some(v => v == 5)) ? b : null), null)], [[], null]).reduce((t, n, i, a) => i ? t * n.reduce((t2, r) => t2 + r.reduce((t3, c) => a[0].includes(c) ? t3 : t3 + c, 0), 0) : t * n[n.length - 1], 1);
// Day 4 Part 2
([ns, ...bs] = fs.readFileSync("./input.txt", "utf-8").trim().split("\n\n").map((v, i) => i ? v.split("\n").map(r => r.trim().split(/ +/g).map(Number)) : v.split(",").map(Number)))[0].reduce(([u, w], n) => w ? [u, w] : [u.concat(n), bs.filter((b) => (b.map(r => r.reduce((t, c) => u.includes(c) ? t + 1 : t, 0)).some(v => v == 5) || [...Array(5)].map((_, c) => [...Array(5)].reduce((t, _, r) => u.includes(b[r][c]) ? t + 1 : t, 0)).some(v => v == 5))).length === bs.length - 1 ? bs.find((b) => !(b.map(r => r.reduce((t, c) => u.includes(c) ? t + 1 : t, 0)).some(v => v == 5) || [...Array(5)].map((_, c) => [...Array(5)].reduce((t, _, r) => u.includes(b[r][c]) ? t + 1 : t, 0)).some(v => v == 5)) && (b.map(r => r.reduce((t, c) => (u.includes(c) || c == n) ? t + 1 : t, 0)).some(v => v == 5) || [...Array(5)].map((_, c) => [...Array(5)].reduce((t, _, r) => (u.includes(b[r][c]) || b[r][c] == n) ? t + 1 : t, 0)).some(v => v == 5))) : null], [[], null]).reduce((t, n, i, a) => i ? t * n.reduce((t2, r) => t2 + r.reduce((t3, c) => a[0].includes(c) ? t3 : t3 + c, 0), 0) : t * n[n.length - 1], 1);
// Day 5 Part 1
[...fs.readFileSync("./input.txt", "utf-8").trim().split("\n").map(l => l.split(" -> ").map(c => c.split(",").map(Number))).filter(l => l[0][0] == l[1][0] || l[0][1] == l[1][1]).reduce((g, l) => [...Array(l[0][0] == l[1][0] ? Math.abs(l[0][1] - l[1][1]) + 1 : Math.abs(l[0][0] - l[1][0]) + 1)].reduce((g2, _, i) => g2.set((k = `${l[0][0] == l[1][0] ? l[0][0] : i + Math.min(l[0][0], l[1][0])}-${l[0][0] == l[1][0] ? i + Math.min(l[0][1], l[1][1]) : l[0][1]}`), (g2.get(k) ?? 0) + 1), g), new Map()).values()].reduce((t, v) => v >= 2 ? t + 1 : t, 0);
// Day 5 Part 2
[...fs.readFileSync("./input.txt", "utf-8").trim().split("\n").map(l => l.split(" -> ").map(c => c.split(",").map(Number))).reduce((g, l) => l[0][0] == l[1][0] || l[0][1] == l[1][1] ? [...Array(l[0][0] == l[1][0] ? Math.abs(l[0][1] - l[1][1]) + 1 : Math.abs(l[0][0] - l[1][0]) + 1)].reduce((g2, _, i) => g2.set((k = `${l[0][0] == l[1][0] ? l[0][0] : i + Math.min(l[0][0], l[1][0])}-${l[0][0] == l[1][0] ? i + Math.min(l[0][1], l[1][1]) : l[0][1]}`), (g2.get(k) ?? 0) + 1), g) : [...Array(Math.abs(l[0][0] - l[1][0]) + 1)].reduce((g2, _, d) => g2.set((k = `${l[0][0] < l[1][0] ? l[0][0] + d : l[0][0] - d}-${l[0][1] < l[1][1] ? l[0][1] + d : l[0][1] - d}`), (g2.get(k) ?? 0) + 1), g), new Map()).values()].reduce((t, v) => v >= 2 ? t + 1 : t, 0);
// Day 6 Part 1

// Day 6 Part 2
