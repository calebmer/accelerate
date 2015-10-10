# accelerate

Oh no…another library to do database migrations. [Aren’t](http://sqitch.org) [there](https://github.com/mattes/migrate) [already](https://www.npmjs.com/package/migrate) [like](https://bitbucket.org/liamstask/goose) [a](https://www.npmjs.com/package/db-migrate) [thousand](https://github.com/tanel/dbmigrate) [of](https://github.com/BurntSushi/migration) [those](http://docs.sequelizejs.com/en/latest/docs/migrations/) [out](https://github.com/DavidHuie/gomigrate) [there](https://github.com/rubenv/sql-migrate)?

Yep.

But they all suck.

I have spent a lot of times hitting my head against a slew of opinionated database migration tools until I finally threw my hands in the air and built my own.

## Drivers
While `accelerate` is designed for database migrations; it can migrate, sorry accelerate, *any* system which will take a script as an input. All it needs is a driver which is insanely easy to [implement](https://raw.githubusercontent.com/calebmer/accelerate/master/lib/drivers/driver.js). Traditional SQL migrations, MongoDB migrations, HTTP migrations, Taco migrations. I’m not joking when I say anything, the tests migrate a *string*.

If you want a driver that is not currently supported, submit an [issue](https://github.com/calebmer/accelerate/issues/new) and I can add it.

### Supported Drivers
- Postgres
	- state: Stored in the `accelerate.state` table
	- example: `postgresql://localhost:5432/database`

Wow…that’s sad. Currently, I only need it for Postgres database work, but you need it for more, so submit those [issues](https://github.com/calebmer/accelerate/issues/new)!

## Motions
In `accelerate` land, every script is called a “motion”. A motion can either add to or subtract from the target.

### Organizing Your Motions
Your motions will live in a special directory that you pick! At the root of this directory you *must* have two template files, one for addition and one for subtraction.

#### Template File
Your template file is your blueprint for future motions. The text inside these files will be copied to every new motion you create. Your template file name must also adhere to a special syntax which defines things about how your motions will be named.

Following is how to name your template file:

```
[version]-template.(add|sub)[extension]
```

Wow, that’s really confusing. Well that’s because it’s super configurable! I’m not going to make you name your templates in any certain way, you be you.

Here’s a couple of sample template name flavors and what their corresponding motion file names would be:

**Plain**
```
xxx-template.add
xxx-template.sub
001-lorem.add
001-lorem.sub
002-ipsum.add
002-ipsum.sub
```

**Semversioned**
```
x.x.x-template.add
x.x.x-template.sub
0.0.1-lorem.add
0.0.1-lorem.sub
2.1.5-ipsum.add
2.1.5-ipsum.sub
```

**Extended**
```
xxx-template.add.sql
xxx-template.sub.sql
001-lorem.add.sql
001-lorem.sub.sql
002-ipsum.add.sql
002-ipsum.sub.sql
```

**A Whole new Seperator**
```
xxx_template.add
xxx_template.sub
001_lorem.add
001_lorem.sub
002_ipsum.add
002_ipsum.sub
```

One gotcha is some people might want to do the following with a semantic versioning style: `0.0.10`, when their template name is this `x.x.x`. That breaks your motions alphabetical sorting order, so if you want `0.0.10` make sure your template is named `x.x.xx`.

Want a naming flavor you can’t have with the current system, you know what [to do](https://github.com/calebmer/accelerate/issues/new).

## Usage
```

  Usage: accelerate [options] [command]


  Commands:

    ls             list all motions to be used
    create <name>  create a new motion named <name> using the template
    add [n]        add n motions (default n=1)
    sub [n]        subtract n motions (default n=1)
    goto <n>       go to the nth motion
    redo           subtract than add the last motion
    up             add all remaining motions
    down           subtract all previous motions
    reset          subtract then add all previous motions

  Options:

    -h, --help              output usage information
    -V, --version           output the version number
    -t, --target [url]      the targeted url to accelerate
    -d, --directory [path]  the directory holding the motions


```

Look at all those sweet sweet commands, gives me goosebumps.

There description hopefully makes them self explanatory, just remeber to always include the target parameter (directory is `.` by default). The target parameter will be your url (see supported drivers above for examples).

## Twitter
If you like `accelerate` FOLLOW ME ON TWITTER, I’m [@calebmer](https://twitter.com/@calebmer). See you there 👍

Awesome.
