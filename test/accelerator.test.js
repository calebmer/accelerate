'use strict'

let Assert = require('assert')
let Accelerator = require('../lib/accelerator')

let value = null
let status = null

let driver = {
  getStatus() { return status },
  setStatus(newStatus) { status = newStatus },
  execute(motion) { value = motion(value) }
}

let motions = [{
  add: () => 'hello',
  sub: () => null
}, {
  add: v => `${v} world`,
  sub: v => v.slice(0, -6)
}, {
  add: v => v.replace(/o/g, 'u'),
  sub: v => v.replace(/u/g, 'o')
}, {
  add: v => v.toUpperCase(),
  sub: v => v.toLowerCase()
}, {
  add: v => v.replace(/L/g, 'l'),
  sub: v => v.replace(/l/g, 'L')
}]

let values = [
  null,
  'hello',
  'hello world',
  'hellu wurld',
  'HELLU WURLD',
  'HEllU WURlD'
]

let accelerator = new Accelerator(driver, motions)

function testHelper(steps) {
  return () => steps.reduce(
    (previousValue, currentValue, index) => previousValue.then(() => {
      if (typeof currentValue === 'function') { currentValue = currentValue() }
      if (currentValue instanceof Promise) { return currentValue }
      else if (typeof currentValue === 'number') {
        Assert.equal(value, values[currentValue], `${value} === ${values[currentValue]} at trial ${index}`)
        Assert.equal(status || 0, currentValue, `${status} === ${currentValue} at trial ${index}`)
      }
    }),
    Promise.resolve(null)
  )
}

describe('Accelerator', () => {
  afterEach(() => {
    value = null
    status = null
  })

  it('gracefully handles errors')

  describe('move(n)', () => {
    it('will move up', testHelper([
      () => accelerator.move(+1), 1
    ]))

    it('will move down', testHelper([
      () => accelerator.move(+1), 1,
      () => accelerator.move(-1), 0
    ]))

    it('will move n times', testHelper([
      () => accelerator.move(+3), 3,
      () => accelerator.move(-2), 1,
      () => accelerator.move(+4), 5,
      () => accelerator.move(-3), 2
    ]))

    it('will only execute in bounds', testHelper([
      () => accelerator.move(-50), 0,
      () => accelerator.move(+50), 5,
      () => accelerator.move(+50), 5,
      () => accelerator.move(-1), 4,
      () => accelerator.move(-50), 0
    ]))
  })

  describe('goto(n)', () => {
    it('will go to n upwards', testHelper([
      () => accelerator.goto(3), 4,
      () => accelerator.goto(4), 5
    ]))

    it('will go to n downwards', testHelper([
      () => accelerator.goto(4), 5,
      () => accelerator.goto(2), 3,
      () => accelerator.goto(1), 2
    ]))

    it('will go to the upstate', testHelper([
      () => accelerator.goto(4), 5,
      () => accelerator.goto(0), 1
    ]))

    it('will only execute in bounds', testHelper([
      () => accelerator.goto(+50), 5,
      () => accelerator.move(-1), 4,
      () => accelerator.goto(-50), 0,
      () => accelerator.move(+1), 1
    ]))
  })

  describe('add()', () => {
    it('will add one', testHelper([
      () => accelerator.add(), 1,
      () => accelerator.move(-1), 0,
      () => accelerator.add(), 1,
      () => accelerator.add(), 2,
      () => accelerator.move(+2), 4,
      () => accelerator.add(), 5
    ]))
  })

  describe('sub()', () => {
    it('will subtract one', testHelper([
      () => accelerator.sub(), 0,
      () => accelerator.move(+50), 5,
      () => accelerator.sub(), 4,
      () => accelerator.move(+1), 5,
      () => accelerator.sub(), 4,
      () => accelerator.sub(), 3,
      () => accelerator.move(-2), 1,
      () => accelerator.sub(), 0
    ]))
  })

  describe('redo()', () => {
    it('will go one down than one up', testHelper([
      () => accelerator.redo(), 1,
      () => accelerator.redo(), 1,
      () => accelerator.move(+3), 4,
      () => accelerator.redo(), 4
    ]))
  })

  describe('up()', () => {
    it('will run all motions upwards', testHelper([() => accelerator.up(), 5]))

    it('will work at any status', testHelper([
      () => accelerator.move(+3), 3,
      () => accelerator.up(), 5
    ]))
  })

  describe('down()', () => {
    it('will run all motions downwards', testHelper([
      () => accelerator.up(), 5,
      () => accelerator.down(), 0
    ]))

    it('will work at any status', testHelper([
      () => accelerator.move(+3), 3,
      () => accelerator.down(), 0
    ]))
  })

  describe('reset()', () => {
    it('will go down and up to the previous status', testHelper([
      () => accelerator.reset(), 0,
      () => accelerator.up(), 5,
      () => accelerator.reset(), 5,
      () => accelerator.move(-2), 3,
      () => accelerator.reset(), 3
    ]))
  })
})
