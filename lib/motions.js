'use strict'

let Assert = require('assert')
let Path = require('path')
let Fs = require('fs')

let Motions = {
  discover(directory) {
    return Promise.all([
      _readDirectory(directory),
      this._getTemplate(directory)
    ])
    .then(results => {
      let names = results[0]
      let template = results[1]

      Assert(template, 'Directory has a template')

      names = names.filter(n => n.match(template.name.regex)).sort()

      let motionsAdd = []
      let motionsSub = []

      names.forEach(n => {
        let operation = template.name.regex.exec(n)[3]
        if (operation === 'add') { motionsAdd.push(n) }
        if (operation === 'sub') { motionsSub.push(n) }
      })

      let motions = motionsAdd
      .map(add => ({
        addName: add,
        subName: motionsSub.find(sub => _disambiguate(sub) === _disambiguate(add))
      }))
      .filter(m => m.addName && m.subName)

      return Promise.all(motions.map(m =>
        Promise.all([
          _readFile(Path.join(directory, m.addName), 'utf8'),
          _readFile(Path.join(directory, m.subName), 'utf8')
        ])
        .then(results => ({
          name: _disambiguate(m.addName),
          addName: m.addName,
          subName: m.subName,
          add: results[0],
          sub: results[1]
        }))
      ))
    })
  },

  create(directory, name) {
    return Promise.all([
      this.discover(directory),
      this._getTemplate(directory)
    ])
    .then(results => {
      let motions = results[0]
      let template = results[1]

      Assert(template, 'Directory has a template')

      let lastMotion = motions[motions.length - 1]
      let version

      if (lastMotion) {
        version = template.name.regex.exec(lastMotion.addName)[1].split('.').map(parseInt)
      } else {
        version = template.name.version.map(() => 0)
      }

      version[version.length - 1] += 1
      version = version.map((num, i) => _padNumber(num, template.name.version[i]))
      version.forEach((v, i) => Assert.equal(v.length, template.name.version[i], `'${v}' is of length equal to ${template.name.version[i]}`))

      let addName = `${version.join('.')}${template.name.seperator}${name}.add${template.name.extension}`
      let subName = `${version.join('.')}${template.name.seperator}${name}.sub${template.name.extension}`

      return Promise.all([
        _writeFile(Path.join(directory, addName), template.add),
        _writeFile(Path.join(directory, subName), template.sub)
      ])
    })
    .then(() => null)
  },

  _getTemplate(directory) {
    const templateNameRegex = {
      add: /^([x.]+)([\-_ ~]+)template\.add(.*)$/i,
      sub: /^([x.]+)([\-_ ~]+)template\.sub(.*)$/i,
    }

    let template = {}

    return _readDirectory(directory)
    .then(names => {
      let templateName = {
        add: names.find(f => f.match(templateNameRegex.add)),
        sub: names.find(f => f.match(templateNameRegex.sub))
      }

      if (!templateName.add || !templateName.sub) {
        throw new Error(`No valid template in directory ${directory}`)
      }

      Assert.equal(
        templateName.add.replace(templateNameRegex.add, '$1,$2,$3'),
        templateName.sub.replace(templateNameRegex.sub, '$1,$2,$3'),
        `'${templateName.add}' matches '${templateName.sub}'`
      )

      let match = templateNameRegex.add.exec(templateName.add)
      template.name = {}
      template.name.version = match[1].split('.').map(s => s.length)
      template.name.seperator = match[2]
      template.name.extension = match[3]

      let versionRegex = template.name.version.map(l => `\\d{${l}}`).join('\\.')
      template.name.regex = new RegExp(`^(${versionRegex})${_escapeRegExp(template.name.seperator)}(.+)\\.(add|sub)${_escapeRegExp(template.name.extension)}$`, 'i')

      return Promise.all([
        _readFile(Path.join(directory, templateName.add), 'utf8'),
        _readFile(Path.join(directory, templateName.sub), 'utf8')
      ])
    })
    .then(files => {
      template.add = files[0]
      template.sub = files[1]
      return template
    })
  }
}

module.exports = Motions

function _disambiguate(name) {
  return name.replace(/\.(add|sub)/, '')
}

function _padNumber(num, size) {
  let s = new String(num)
  while (s.length < size) { s = '0' + s }
  return s
}

function _escapeRegExp(s) {
  return s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

function _readDirectory(path) {
  return new Promise((resolve, reject) =>
    Fs.readdir(path, (error, files) => {
      if (error) { return reject(error) }
      resolve(files)
    })
  )
}

function _readFile(path, options) {
  return new Promise((resolve, reject) =>
    Fs.readFile(path, options, (error, file) => {
      if (error) { return reject(error) }
      resolve(file)
    })
  )
}

function _writeFile(path, file, options) {
  return new Promise((resolve, reject) =>
    Fs.writeFile(path, file, options, error => {
      if (error) { return reject(error) }
      resolve()
    })
  )
}
