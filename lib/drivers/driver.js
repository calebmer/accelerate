'use strict'

class Driver {
  constructor(url) { this.url = url }
  getStatus() { throw new Error('Not implemented') }
  setStatus() { throw new Error('Not implemented') }
  execute() { throw new Error('Not implemented') }
}

module.exports = Driver
