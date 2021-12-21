const fs = require('fs');

const input = fs.readFileSync('./input.txt')
    .toString("utf8")
    .split('\n');

const INSTRUCTION_REGEX = /(forward|up|down) (\d+)/;


input.map(instr => {
    const [_, direction, magnitude] = instr.match(INSTRUCTION_REGEX);
    return {
        direction,
        magnitude: parseInt(magnitude, 10),
    };
}).reduce((acc, instruction) => {
    const { direction, magnitude } = instruction;
    switch (direction) {
        case 'forward':
            acc.forward += magnitude;
            break;
        case 'up':
            acc.depth -= magnitude;
            break;
        case 'down':
            acc.depth += magnitude;
            break;
    };
    console.log(`Move: ${direction} ${magnitude}`);
    console.log(`Forward: ${acc.forward}; Depth: ${acc.depth}`);
    console.log(`Product ${acc.forward * acc.depth}`);

    return acc;
}, {
    depth: 0,
    forward: 0,
});

// 861x1940 2801 too low