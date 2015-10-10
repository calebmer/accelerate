'use strict'

let Assert = require('assert')

const PROTOCOL_REGEX = /^(.*):\//

let Drivers = {
  get(url) {
    let result = PROTOCOL_REGEX.exec(url)
    Assert(result, 'Driver supports url syntax')

    let Driver = require(`./${result[1]}`)

    return new Driver(url)
  }
}

module.exports = Drivers
