'use strict'

let Postgres = require('pg')
let Driver = require('./driver')

const QUERIES = {
createSchema:
'CREATE SCHEMA IF NOT EXISTS accelerate',

createTable:
`CREATE TABLE IF NOT EXISTS accelerate.state (
  status   integer,
  inserted timestamp DEFAULT current_timestamp
)`,

selectStatus:
` SELECT status
    FROM accelerate.state
ORDER BY inserted DESC
   LIMIT 1`,

setStatus:
`INSERT INTO accelerate.state (status) VALUES ($1)`
}

class PostgresDriver extends Driver {
  constructor(url) {
    super(url)
    this.reconnect()
  }

  reconnect() {
    let client
    this.client = new Promise((resolve, reject) => {
      Postgres.connect(this.url, (error, c) => {
        if (error) { return reject(error) }
        client = c
        resolve()
      })
    })
    .then(() => _query(client, QUERIES.createSchema))
    .then(() => _query(client, QUERIES.createTable))
    .then(() => client)
  }

  _query(query, values) {
    let result = this.client.then(client => _query(client, query, values))
    result.catch(() => this.reconnect())
    return result
  }

  getStatus() {
    return this._query(QUERIES.selectStatus)
    .then(results => results.rows[0] ? results.rows[0].status : null)
  }

  setStatus(newStatus) { return this._query(QUERIES.setStatus, [newStatus]) }
  execute(motion) { return this._query(motion) }
}

module.exports = PostgresDriver

function _query(client, query, values) {
  return new Promise((resolve, reject) => {
    client.query(query, values || [], (error, results) => {
      if (error) { return reject(error) }
      resolve(results)
    })
  })
}
