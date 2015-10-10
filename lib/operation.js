'use strict'

let Operation = {
  add: {
    name: 'up',
    symbol: Symbol('up'),
    unit: +1
  },
  sub: {
    name: 'down',
    symbol: Symbol('down'),
    unit: -1
  },

  get(n) { return n >= 0 ? this.add : this.sub }
}

module.exports = Operation
