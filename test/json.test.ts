import { describe, expect, test } from '@jest/globals';
import { JSONParser, readNumberTo } from '@web/json';

function testObject(v: string) {
  test('parse object', () => {
    const parser = new JSONParser();
    expect(parser.parse(v)).toEqual(JSON.parse(v));
  });
}

describe('parse json', () => {
  test('readNumberTo', () => {
    expect(readNumberTo('123', 0)).toBe(2);
  });

  test('readNumberTo', () => {
    expect(readNumberTo('foo123xxx', 3)).toBe(5);
  });

  testObject(`
  {
  }
  `);

  testObject(`
  {
    "foo": "bar"
  }
  `);

  testObject(`
  {
    "foo": "bar",
    "bar": "foo"
  }
  `);

  testObject(`
  {
    "foo": "bar",
    "bar": "foo",
    "apple": "banana"
  }
  `);

  testObject(`
  {
    "foo": "bar",
    "bar": "foo",
    "apple": "banana",
    "cc": "www"
  }
  `);

  testObject(`
  {
    "foo": 12
  }
  `);

  testObject(`
  {
    "foo": 12,
    "bar": 34
  }
  `);

  testObject(`
  {
    "foo": 12,
    "bar": 34,
    "fff": 34
  }
  `);

  testObject(`
  {
    "foo": 12,
    "bar": 34,
    "fff": 34,
    "fff1": "34",
    "fff2": "34",
    "fff3": 34
  }
  `);

  testObject(`
  {
    "foo": 12,
    "bar": 34,
    "fff": 34,
    "fff1": "34",
    "fff2": "34",
    "fff3": 34,
    "fff3x": 343,
    "fff3xx": "343",
    "fff3xxx": 343,
    "fff3xxxx": "343"
  }
  `);

  testObject(`
  {
    "foo": true,
    "bar": false,
    "zzz": null
  }
  `);

  testObject(`
  {
    "zzz": "zzz",
    "foo": true,
    "zzz": "zzz",
    "zzz": "zzz",
    "bar": false,
    "vvvv": "vvvv",
    "zzz": null,
    "qqq": 123,
    "fff": 0
  }
  `);

  testObject(`
  {
    "okdc": "zzz",
    "foo": true,
    "zzzdcijv": "zzz",
    "zzzsxuhc": "zzz",
    "bar": false,
    "barz": false,
    "sxiji": false,
    "vvvv": "vvvv",
    "zzzsxhu": null,
    "qqq": 123,
    "aisxji": 123,
    "fff": 0
  }
  `);

  testObject(`
  {
    "foo": {}
  }
  `);

  testObject(`
  {
    "foo": {
      "bar": 1
    }
  }
  `);

  testObject(`
  {
    "foo": {
      "bar": 1,
      "baz": 2,
      "vvv": 3,
      "qqq": {
        "xxx": {
          "yyy": {
            "qqq": 1
          }
        }
      }
    }
  }
  `);
});
