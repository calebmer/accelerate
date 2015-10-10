'use strict'

let Assert = require('assert')
let Motions = require('../lib/motions')

describe('Motions', () => {
  describe('discover()', () => {
    it('will find everything', () =>
      Motions.discover('./test/motions/standard')
      .then(motions =>
        Assert.deepEqual(motions, [{
          name: '001-lorem-ipsum',
          addName: '001-lorem-ipsum.add',
          subName: '001-lorem-ipsum.sub',
          add: '001,lorem-ipsum,add\n',
          sub: '001,lorem-ipsum,sub\n'
        }, {
          name: '002-hello-world',
          addName: '002-hello-world.add',
          subName: '002-hello-world.sub',
          add: '002,hello-world,add\n',
          sub: '002,hello-world,sub\n'
        }, {
          name: '004-motion',
          addName: '004-motion.add',
          subName: '004-motion.sub',
          add: '004,motion,add\n',
          sub: '004,motion,sub\n'
        }, {
          name: '999-thing',
          addName: '999-thing.add',
          subName: '999-thing.sub',
          add: '999,thing,add\n',
          sub: '999,thing,sub\n'
        }])
      )
    )

    it('will find all the semantic versioned things', () =>
      Motions.discover('./test/motions/semantic')
      .then(motions =>
        Assert.deepEqual(motions, [{
          name: '0.0.01-lorem-ipsum',
          addName: '0.0.01-lorem-ipsum.add',
          subName: '0.0.01-lorem-ipsum.sub',
          add: '0.0.01,lorem-ipsum,add\n',
          sub: '0.0.01,lorem-ipsum,sub\n'
        }, {
          name: '3.2.01-hello-world',
          addName: '3.2.01-hello-world.add',
          subName: '3.2.01-hello-world.sub',
          add: '3.2.01,hello-world,add\n',
          sub: '3.2.01,hello-world,sub\n'
        }])
      )
    )

    it('will find everything with a different seperator', () =>
      Motions.discover('./test/motions/seperator')
      .then(motions =>
        Assert.deepEqual(motions, [{
          name: '001_lorem-ipsum',
          addName: '001_lorem-ipsum.add',
          subName: '001_lorem-ipsum.sub',
          add: '001,lorem-ipsum,add\n',
          sub: '001,lorem-ipsum,sub\n'
        }])
      )
    )

    it('will find everything with a matching extension', () =>
      Motions.discover('./test/motions/extension')
      .then(motions =>
        Assert.deepEqual(motions, [{
          name: '010-foo.csv',
          addName: '010-foo.add.csv',
          subName: '010-foo.sub.csv',
          add: '010,foo,add\n',
          sub: '010,foo,sub\n'
        }])
      )
    )
  })

  describe('create()', () => {
    it('copies the template')
    it('increments the version')
    it('increments the semantic version')
    it('matches the seperator')
    it('keeps the extension')
  })
})
