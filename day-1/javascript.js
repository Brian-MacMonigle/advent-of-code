const fs = require('fs');

const input = fs.readFileSync('./input.txt')
    .toString("utf8")
    .split('\n')
    .map(value => parseInt(value, 10));

class Counter {
    increased = 0;
    decreased = 0;
    last = undefined;

    updateLast = (fn) => (value) => {
        const res = fn(value);
        this.last = value;
        return res;
    }

    getIncreased() {
        return this.increased;
    }

    getDecreased() {
        return this.decreased;
    }

    handle = this.updateLast((value) => {
        if (this.last === undefined) {
            return `${value} (N/A - no previous measurement)`;
        }

        if (this.last < value) {
            this.increased++;
            return `${value} (increased)`;
        }

        if (this.last > value) {
            this.decreased++;
            return `${value} (decreased)`;
        }
    })
}

const counter = new Counter();

input.forEach(value => console.log(counter.handle(value)));

console.log({
    increased: counter.increased,
    decreased: counter.decreased,
});