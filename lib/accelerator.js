'use strict'

let EventEmitter = require('events').EventEmitter
let Operation = require('./operation')

class Accelerator extends EventEmitter {
  constructor(driver, motions) {
    super()
    let accelerate = this
    this.motions = motions
    this.driver = {
      __proto__: driver,
      getStatus() { return Promise.resolve(super.getStatus()).then(s => typeof s === 'number' ? s : 0) },
      setStatus(newStatus) { return Promise.resolve(super.setStatus(accelerate._inBounds(newStatus))).then(() => null) },
      execute(motion) { return Promise.resolve(super.execute(motion)) }
    }
  }

  _inBounds(n) {
    let min = 0
    let max = this.motions.length
    if (n < min) { return min }
    if (n > max) { return max }
    return n
  }

  _execute(start, finish) {
    start = this._inBounds(start)
    finish = this._inBounds(finish)
    let operation = Operation.get(finish - start)
    let shift = operation === Operation.sub ? 1 : 0
    start -= shift
    finish -= shift

    let motions = this.motions.map(m => m[operation.name])
    let execution = Promise.resolve(null)
    let status = start

    for (let i = start; i !== finish; i += operation.unit) {
      execution = execution.then(() => this.driver.execute(motions[i]))
      execution.then(() => status += operation.unit)
      execution.then(() => this.emit('motion', this.motions[i], operation))
    }

    return new Promise((resolve, reject) => {
      execution.then(() =>
        this.driver.setStatus(status + shift)
        .then(() => resolve())
      )
      execution.catch(error =>
        this.driver.setStatus(status + shift)
        .then(() => reject(error))
        .catch(reject)
      )
    })
  }

  move(n) {
    let start
    let finish
    return this.driver.getStatus()
    .then(s => {
      start = s
      finish = this._inBounds(start + n)
    })
    .then(() => this._execute(start, finish))
  }

  goto(finish) {
    finish += 1
    return this.driver.getStatus()
    .then(start => this._execute(start, finish))
  }

  add() { return this.move(Operation.add.unit) }
  sub() { return this.move(Operation.sub.unit) }
  redo() { return this.sub().then(() => this.add()) }
  up() { return this.move(Operation.add.unit * Infinity) }
  down() { return this.move(Operation.sub.unit * Infinity) }

  reset() {
    let status
    return this.driver.getStatus()
    .then(s => status = s)
    .then(() => this._execute(status, 0))
    .then(() => this._execute(0, status))
  }
}

module.exports = Accelerator
