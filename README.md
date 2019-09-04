# H.A.T - Home Automation Tools

This is my repo containing tools written for my IoT experiements. The idea is
to make some generic small tools to support different devices, that can be used
in many contexts depending on your need.

HATs goal is that the tools will be:
 - Small/Simple - doing one thing well
 - Prototype - fast iterations

## hat-mibcs
`hat-mibcs` is a tool for reading data from "Xiaomi MiScale (MIBCS)". Once
started it will output a JSON string with the data it receives via Bluetooth.

### Example
The following is a measurement of me (and our dog?), a few days ago. As you see
it provides `weight` and `impedance` - values you can do for whatever you need.
```
$ sudo hat-mibcs -s 0
{"source":"hat-mibcs","address":"EF:FB:0D:B1:43:97","datetime":"2019-05-13 16:03:35","weight":95.4,"impedance":397}
{"source":"hat-mibcs","address":"EF:FB:0D:B1:43:97","datetime":"2019-05-13 20:19:06","weight":95.4,"impedance":397}
{"source":"hat-mibcs","address":"EF:FB:0D:B1:43:97","datetime":"2019-05-13 20:27:37","weight":94.799995,"impedance":394}
{"source":"hat-mibcs","address":"EF:FB:0D:B1:43:97","datetime":"2019-05-13 22:44:59","weight":94.799995,"impedance":401}
{"source":"hat-mibcs","address":"EF:FB:0D:B1:43:97","datetime":"2019-05-14 06:11:32","weight":93.9,"impedance":440}
{"source":"hat-mibcs","address":"EF:FB:0D:B1:43:97","datetime":"2019-05-14 17:09:12","weight":3.3999999,"impedance":null}
```

  `-s 0` means that `hat-mibcs` will wait forever for data. If started without
  that parameter, it will listen for data for a minute before existing.

If you want to integrate this with Home Assistant or OpenHab you can utilize
MQTT. If you install mosquitto, you could pipe the output from `hat-mibcs`
directly to `mosquitto_pub` like this:
```
$ hat-mibcs -s 0 | mosquitto_pub -l -t "miscale"
```

This will publish the measurements to MQTT under the `miscale` topic, where
Home Assistant, OpenHab or Node-Red can do further processing of data.

## hat-miflora
`hat-miflora` is a tool for reading data from Xiaomi Miflora sensor.

NOTICE: This have only been tested with version 3.1.8+ firmware

It will only fetch data once. If you want to have it gather every hour or
similar, then add it to your crontab.

### Examples

Scan of Mifloras:
```
$ hat-miflora scan
- addr: "C4:7C:8D:65:BD:8B"
  name: "Flower care"
  alias: "Flower care"
- addr: "C4:7C:8D:67:C2:8B"
  name: "Flower care"
  alias: "Flower care"
```

If you wish, you can get the same output as JSON.
```
$ hat-miflora --json scan
{"devices":[{"addr":"C4:7C:8D:67:C2:8B","name":"Flower care","alias":"Flower care"},{"addr":"C4:7C:8D:65:BD:8B","name":"Flower care","alias":"Flower care"}]}
```

Reading data from a device
```
$ hat-miflora --json read C4:7C:8D:67:C2:8B
{"address":"C4:7C:8D:67:C2:8B","battery_pct":98,"firmware_version":"3.1.9","temperature":23.9,"lux":112,"moisture":0,"conductivity":0}
```

For help and info:
```
$ hat-miflora
hat-miflora 0.4.0

USAGE:
    hat-miflora [FLAGS] <SUBCOMMAND>

FLAGS:
    -h, --help          Prints help information
    -j, --json          Output JSON
    -H, --no-headers    Don't show headers (ignored for JSON output)
    -V, --version       Prints version information

SUBCOMMANDS:
    blink            Make Miflora device blink
    help             Prints this message or the help of the given subcommand(s)
    history          Read historical data from Miflora device
    history-clear    Clear historical data from Miflora device
    history-count    Read number of historical records from Miflora Device
    read             Read realtime data from Miflora device
    scan             Scan for Miflora Devices
```
