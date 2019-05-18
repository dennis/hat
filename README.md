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
$ hat-mimcs -s 0 | mosquitto_pub -l -t "miscale"
```

This will publish the measurements to MQTT under the `miscale` topic, where
Home Assistant, OpenHab or Node-Red can do further processing of data.
