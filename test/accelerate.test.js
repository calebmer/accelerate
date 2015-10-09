'use strict'

let Assert = require('assert')
let Accelerate = require('../lib/accelerate')

let value = null
let status = null

let driver = {
  getStatus() { return status },
  setStatus(newStatus) { status = newStatus },
  execute(motion) { value = motion(value) }
}

let motions = [{
  up:   () => 'hello',
  down: () => null
}, {
  up:   v => `${v} world`,
  down: v => v.slice(0, -6)
}, {
  up:   v => v.replace(/o/g, 'u'),
  down: v => v.replace(/u/g, 'o')
}, {
  up:   v => v.toUpperCase(),
  down: v => v.toLowerCase()
}, {
  up:   v => v.replace(/L/g, 'l'),
  down: v => v.replace(/l/g, 'L')
}]

let values = [
  null,
  'hello',
  'hello world',
  'hellu wurld',
  'HELLU WURLD',
  'HEllU WURlD'
]

let accelerate = new Accelerate(driver, motions)

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

describe('Accelerate', () => {
  afterEach(() => {
    value = null
    status = null
  })

  describe('move(n)', () => {
    it('will move up', testHelper([
      () => accelerate.move(+1), 1
    ]))

    it('will move down', testHelper([
      () => accelerate.move(+1), 1,
      () => accelerate.move(-1), 0
    ]))

    it('will move n times', testHelper([
      () => accelerate.move(+3), 3,
      () => accelerate.move(-2), 1,
      () => accelerate.move(+4), 5,
      () => accelerate.move(-3), 2
    ]))

    it('will only execute in bounds', testHelper([
      () => accelerate.move(-50), 0,
      () => accelerate.move(+50), 5,
      () => accelerate.move(+50), 5,
      () => accelerate.move(-1), 4,
      () => accelerate.move(-50), 0
    ]))
  })

  describe('goto(n)', () => {
    it('will go to n upwards', testHelper([
      () => accelerate.goto(3), 4,
      () => accelerate.goto(4), 5
    ]))

    it('will go to n downwards', testHelper([
      () => accelerate.goto(4), 5,
      () => accelerate.goto(2), 3,
      () => accelerate.goto(1), 2
    ]))

    it('will go to the upstate', testHelper([
      () => accelerate.goto(4), 5,
      () => accelerate.goto(0), 1
    ]))

    it('will only execute in bounds', testHelper([
      () => accelerate.goto(+50), 5,
      () => accelerate.move(-1), 4,
      () => accelerate.goto(-50), 0,
      () => accelerate.move(+1), 1
    ]))
  })

  describe('up()', () => {
    it('will run all motions upwards', testHelper([() => accelerate.up(), 5]))

    it('will work at any status', testHelper([
      () => accelerate.move(+3), 3,
      () => accelerate.up(), 5
    ]))
  })

  describe('down()', () => {
    it('will run all motions downwards', testHelper([
      () => accelerate.up(), 5,
      () => accelerate.down(), 0
    ]))

    it('will work at any status', testHelper([
      () => accelerate.move(+3), 3,
      () => accelerate.down(), 0
    ]))
  })

  describe('redo()', () => {
    it('will go one down than one up', testHelper([
      () => accelerate.redo(), 1,
      () => accelerate.redo(), 1,
      () => accelerate.move(+3), 4,
      () => accelerate.redo(), 4
    ]))
  })

  describe('reset()', () => {
    it('will go down and up to the previous status', testHelper([
      () => accelerate.reset(), 0,
      () => accelerate.up(), 5,
      () => accelerate.reset(), 5,
      () => accelerate.move(-2)//, 3,
      // () => accelerate.reset(), 3
    ]))
  })
})
