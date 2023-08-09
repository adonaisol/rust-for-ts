import fs from 'fs'

// map over a list containing 1, 2, 3 and increase each value by 1 
console.log([1,2,3].map(i => i+1))
// read file called lines and print out each line individually
fs.readFileSync('lines').toString().split('\n').forEach((line, idx) => idx % 2 === 0 && console.log(line))

fs.readFileSync('lines').toString()
    .split('\n')
    .filter((_, idx) => idx % 2 === 0)
    .filter((_, idx) => idx >= 2 && idx <= 4)
    .forEach(line => console.log(line))

enum Color {
    Red, Green, Blue, Yellow
}

function printColor(color: Color) {
    switch (color) {
        case Color.Green:
            console.log("green")
            break
        case Color.Blue:
            console.log('blue')
            break
        case Color.Red:
            console.log('red')
            break
    }
}

printColor(Color.Blue)


/// 

type Custom = {
    age: number
    name: string
}

type Item = number | string | Custom

const append = (items: Item[]) => {
    items.push('Hello Fem!')
}

let items: number[] = [1, 2, 3]
append(items)

console.log(items)

type Foo = {
    bar?: string
}

// const f: Foo = { bar: undefined}

const fn = (input: number | undefined): number | undefined => {
    // if(typeof input === undefined) 
    //     return 0

    // return (input ?? 0) * 5
    return input === undefined ? undefined : input * 5
}