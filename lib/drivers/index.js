
let Drivers = {
  get(driver) { return this[driver] },
  has(driver) { return !!this.get(driver) },

  postgres: require('./postgres')
}

module.exports = Drivers
