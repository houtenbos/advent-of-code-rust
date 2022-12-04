const crypto = require('crypto');

const test1 = "abcdef609043";
const test2 = "pqrstuv1048970";

// console.log(findHash("abcdef"));
// console.log(findHash("prqstuv"));
console.log(findHash("ckczppom"));


function findHash(string){
    let i = 0;
    while( true ){
        const h = hash( string + i );
        if( isCorrectHash(h) )break;
        i++;
    }
    return string+i;
}

function hash(data){
    return crypto.createHash('md5').update(data).digest("hex");
}

function isCorrectHash(hash){
    return hash.slice(0, 6) === '000000';
}
