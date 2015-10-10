'use strict'

let Operation = {
  add: {
    name: 'add',
    symbol: Symbol('add'),
    unit: +1
  },
  sub: {
    name: 'sub',
    symbol: Symbol('sub'),
    unit: -1
  },

  get(n) { return n >= 0 ? this.add : this.sub }
}

module.exports = Operation
