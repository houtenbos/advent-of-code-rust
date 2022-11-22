const fs = require('fs');


fs.readFile('./input.txt', 'utf8', (e, file) => {
    // const test3 = '^v^v^v^v^v';

    const map = new Map();
    const location1 = [0,0];
    const location2 = [0,0];
    
    bringPresent(location1, map);
    bringPresent(location2, map);

    let isRobo = false;
    for( const char of file ){
        const location = isRobo ? location2 : location1;

        move(char, location);
        bringPresent(location, map);

        isRobo = !isRobo;
    }

    console.log(map.size);
});



function bringPresent(location, map){
    const key = '' + location[0] + ',' + location[1];
    const house = map.get(key);
    if( house )
        map.set(key, house + 1);
    else
        map.set(key, 1);
}

function move(char, location){
    switch(char){
        case '<':
            location[0] -= 1;
            break;
        case '>':
            location[0] += 1;
            break;
        case '^':
            location[1] += 1;
            break;
        case 'v':
            location[1] -= 1;
            break;
         default:
               break;
               // throw new Error('Invalid move' + char );
    }
}
