const fs = require('fs');

const l = 20
const b = 3
const h = 11
console.log(Math.max(l, b, h));
console.log(l + b + h - Math.max(l, b, h))
console.log(2*(l + b + h - Math.max(l, b, h)));

fs.readFile('./input.txt', 'utf8', (e, file) => {
    let paper = 0;
    let ribbon = 0;
    const lines = file.split('\n')
    for( const line of lines ){
        const [l, b, h] = line.split('x');
        if(isNaN(+l * +b * +h)) continue;
        // paper += 2 * l * b + 2 * l * h + 2 * b * h + Math.min(+l * +b, +l * +h, +b * +h);
        ribbon += l * b * h + 2 * (+l + +b + +h - Math.max(l, b, h));
    }
    console.log(paper)

    console.log(ribbon)
});
