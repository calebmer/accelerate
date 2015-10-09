'use strict'

let Direction = {
  up: {
    name: 'up',
    symbol: Symbol('up'),
    unit: +1
  },
  down: {
    name: 'down',
    symbol: Symbol('down'),
    unit: -1
  },

  get(n) { return n >= 0 ? this.up : this.down }
}

module.exports = Direction
