const fs = require('fs');
const data = fs.readFileSync('input.txt','utf8');
const positions = new Set();
let head = [0, 0];
let tail = [0, 0];

const rope = [];
for( let i = 0; i<10; i++){
    rope.push([0, 0]);
}
console.log(rope);

const DIR = {
    R: [1, 0],
    L: [-1, 0],
    U: [0, 1],
    D: [0, -1 ]
}


for( const line of data.split('\n') ){
    if( line ){
        const [dir, steps] = line.split(' ');
        moveRope(rope, DIR[dir], steps);
    }
};
console.log(positions.size);

function moveHead(head, tail, direction, steps){
    if( steps == 0 )
        return;

    console.log('move', direction);
    head[0] += direction[0];
    head[1] += direction[1];
    moveTail(head, tail);
    // safe tail location
    positions.add(`${tail[0]},${tail[1]}`);
    moveHead(head, tail, direction, steps -1);
}

function moveRope(rope, direction, steps){
    if( steps == 0 )
        return;

    const t = rope.length - 1;
    rope[0][0] += direction[0];
    rope[0][1] += direction[1];
    for( let i = 0; i < t; i++){
        moveTail(rope[i], rope[i+1]);
    }
    // safe tail location
    positions.add(`${rope[t][0]},${rope[t][1]}`);
    moveRope(rope, direction, steps -1);
}

function moveTail(head, tail){
    const dx = head[0] - tail[0];
    const dy = head[1] - tail[1];
    if( Math.abs(dx) <= 1 && Math.abs(dy) <= 1 ){
        // do nothing
    }else if( dx == 0 ){
        // move vertical
        tail[1] += 1 * dy/Math.abs(dy);
    }else if( dy == 0 ){
        // move horizontal
        tail[0] += 1 * dx/Math.abs(dx);
    }else{
        // move diagonal
        tail[0] += 1 * dx/Math.abs(dx);
        tail[1] += 1 * dy/Math.abs(dy);
    }
    // test
    {
        const dx = head[0] - tail[0];
        const dy = head[1] - tail[1];
        if( Math.abs(dx) > 1 || Math.abs(dy) > 1 ){
            throw Error(`Invalid tail position. h: ${head}, t: ${tail}`);
        }
    }
}

