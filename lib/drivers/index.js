
let Drivers = {
  postgres: require('./postgres'),

  get(driver) { return this[driver] }
}

module.exports = Drivers
