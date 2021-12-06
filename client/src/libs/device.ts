import { IPv4 } from 'ip-num/IPNumber'

// *** custom types to use as device properties
enum DeviceType {
  BATTERY,
  INVERTER,
  SENSOR
}

enum ConnectionStatus {
  DISCONNECTED,
  CONNECTED
}

enum DeviceStatus {
  SAFE,
  UNSAFE
}

// hacky way to enforce uint16 in typescript, might be a better way
type PortNumber = number & {_type_: "PortNumber"}
const port = (value: number): PortNumber => {
  if (value < 0 || value > 65535) {
    throw new Error(`${value} is not a valid port.`)
  }

  return value as PortNumber
}

export class DeviceFields {
  field_name: string = ''
  field_value: string | number = ''
}
// ***

// common device properties
export interface Device {
  name: string
  dev_type: DeviceType
  ip_address: IPv4
  port: PortNumber
  conn_status: ConnectionStatus
  dev_status: DeviceStatus
  fields: Array<DeviceFields>
}

// common device functions to manipulate device properties
export class DeviceFunctions {

}

// battery specific properties/functions
export class Battery extends DeviceFunctions implements Device {
  name = "Battery 1"
  dev_type = DeviceType.BATTERY
  ip_address = new IPv4("127.0.0.1")
  port = port(0)
  conn_status = ConnectionStatus.DISCONNECTED
  dev_status = DeviceStatus.SAFE
  fields = [{ field_name: 'Temperature', field_value: '' },
            { field_name: 'Power', field_value: '' }]
}

// inverter specific properties/functions
export class Inverter extends DeviceFunctions implements Device {
  name = "Inverter 1"
  dev_type = DeviceType.INVERTER
  ip_address = new IPv4("127.0.0.1")
  port = port(0)
  conn_status = ConnectionStatus.DISCONNECTED
  dev_status = DeviceStatus.SAFE
  fields = [{ field_name: 'Inverter Field 1', field_value: '' },
            { field_name: 'Inverter Field 2', field_value: '' }]
}

// sensor specific properties/functions
export class Sensor extends DeviceFunctions implements Device {
  name = "Sensor 1"
  dev_type = DeviceType.SENSOR
  ip_address = new IPv4("127.0.0.1")
  port = port(0)
  conn_status = ConnectionStatus.DISCONNECTED
  dev_status = DeviceStatus.SAFE
  fields = []
}
