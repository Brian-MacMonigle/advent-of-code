const fs = require('fs');

const input = fs.readFileSync('./input.txt')
    .toString("utf8")
    .split('\n')
    .map(value => parseInt(value, 10));

const rolling_input = [];

for (let i = 0; i < input.length; i++) {
    rolling_input[i] = (rolling_input[i] || 0) + input[i];
    rolling_input[i + 1] = (rolling_input[i + 1] || 0) + input[i];
    rolling_input[i + 2] = (rolling_input[i + 2] || 0) + input[i];
}
rolling_input.splice(0, 2);
rolling_input.splice(rolling_input.length - 2);

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

rolling_input.forEach(value => console.log(counter.handle(value)));

console.log({
    increased: counter.increased,
    decreased: counter.decreased,
});