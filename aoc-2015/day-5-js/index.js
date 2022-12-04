const fs = require('fs');
const data = fs.readFileSync('input.txt','utf8');

const check1 = "ugknbfddgicrmopn"; // true
const check2 = "aaa"; // true
const check3 = "jchzalrnumimnmhp"; // false
const check4 = "haegwjzuvuyypxyu"; // is naughty because it contains the string xy.
const check5 = "dvszwmarrgswjxmb";//  is naughty because it contains only one vowel.

let count = 0;
for( const line of data.split('\n') ){
    if( checkAll(line)){
        count+=1
    }
};
console.log(count);

function checkAll(string){
    return checkVowels(string) && checkDouble(string) && checkForbidden(string);
}

function checkForbidden(string){
    const forbidden = ['ab', 'cd', 'pq', 'xy'];
    let prevC = ''
    for( const c of string){
        if( forbidden.includes(prevC + c)  )return false;
        prevC = c;
    }
    return true;
}

function checkDouble(string){
    let prevC = ''
    for( const c of string){
        if( prevC == c )return true;
        prevC = c;
    }
    return false;
}

function checkPair(string){
    let prevC = ''
    let i = 0;
    for( const c of string ){
        if( i > 0 ){
            const pair = prevC + c;
            string.slice(i-1, i);

        }
        
        prevC = c
        

    }
    return false;
}

function checkVowels(string, min = 3){
    const vowels = "aeiou";
    let count = 0;
    for( const c of string){
        if( vowels.includes(c) ){
            count += 1;
        }
    }
    return count >= min;
}
