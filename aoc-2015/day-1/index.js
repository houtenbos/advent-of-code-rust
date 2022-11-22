const fs = require('fs');

let count = 0;
let index = 0;
fs.readFile('./input.txt', (e, file) => {
    const input = file.toString();
    for( const char of input ){
        if( char == '(')count += 1
        else count -= 1
        index+= 1;
        if( count == -1){
            break;
        }
    }
    console.log(count);
    console.log(index);

});
