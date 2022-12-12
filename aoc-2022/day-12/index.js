const fs = require('fs');
const data = fs.readFileSync('input-test.txt','utf8');
const lines = data.split('\n');
const map = [];
const visitedMap = [];

const DIR = {
    'v': [1, 0],
    '^': [-1, 0],
    '>': [0, 1],
    '<': [0, -1],
}
let start = {x: 0, y: 0};
let end = {x: 0, y: 0};

for( const line of lines ){
    map.push([]);
    visitedMap.push([]);
    for( const c of line ){
        if( c == 'S' ){
            start.x = map.length -1;
            start.y = map[map.length -1].length;
        }
        if( c == 'E' ){
            end.x = map.length -1;
            end.y = map[map.length -1].length;
        }
        map[map.length-1].push(c.charCodeAt());
        visitedMap[map.length-1].push(100000);
    }

}

map[start.x][start.y] = 1000;
map[end.x][end.y] = 122;
console.log(start);
console.log(end);
map.forEach(r => {console.log(r.map(e => String.fromCharCode(e)).join(''))});
let minSteps = 10000;
let fastestPath = [];

// a 
// let queue = [[[], start.x, start.y, 0]];
// while (queue.length > 0) {
//     const [visited, x, y, steps ] = queue.shift();

//     if( x== end.x && y == end.y){
//         console.log('hoi', steps);
//         if( steps < minSteps ){
//             minSteps = steps;
//             fastestPath = visited;
//         }
//         continue;
//     }
//     for( const [d, dir] of Object.entries(DIR) ){
//         const newX = x + dir[0];
//         const newY = y + dir[1];

//         if( newX >= 0 && newX < map.length && newY >= 0 && newY < map[0].length ){
//             if( map[newX][newY] - map[x][y] <= 1 ){
//                 if( visitedMap[newX][newY] > steps ){
//                     visitedMap[newX][newY] = steps;
//                     visited.push([x, y, d]);
//                     queue.push([[...visited], newX, newY, steps + 1]);
//                     // console.log(steps);
//                     // console.log(queue.length);
//                 }else{
//                     continue;
//                 }
//             }
//         }
//     }
// }

// b 
let queue = [[[], end.x, end.y, 0]];
while (queue.length > 0) {
    const [visited, x, y, steps ] = queue.shift();

    if( map[x][y] == 'a'.charCodeAt() ){
        console.log('hoi', steps);
        if( steps < minSteps ){
            minSteps = steps;
            fastestPath = visited;
        }
        continue;
    }
    for( const [d, dir] of Object.entries(DIR) ){
        const newX = x + dir[0];
        const newY = y + dir[1];

        if( newX >= 0 && newX < map.length && newY >= 0 && newY < map[0].length ){
            if( map[x][y] - map[newX][newY] <= 1 ){
                if( visitedMap[newX][newY] > steps ){
                    visitedMap[newX][newY] = steps;
                    visited.push([x, y, d]);
                    queue.push([[...visited], newX, newY, steps + 1]);
                    // console.log(steps);
                    // console.log(queue.length);
                }else{
                    continue;
                }
            }
        }
    }
}

for( const [x, y, d] of fastestPath ){
    map[x][y] = d.charCodeAt();

}
console.log(minSteps);
console.log('--------------------------');

map.forEach(r => {console.log(r.map(e => String.fromCharCode(e)).join(''))});
