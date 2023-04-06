let num = 3;

while (num > -5) {
    setTimeout(() => {
        console.log('Num is', num);
    }, 1000);
    num -= 1;
}

console.log('hi');