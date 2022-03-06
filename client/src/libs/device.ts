// *** custom types to use as device properties
export enum DeviceType {
  Battery = 'Battery',
  Inverter = 'Inverter',
  Sensor = 'Sensor'
}

export enum DeviceTypeIcon {
  Battery = 'battery_full',
  Inverter = 'bolt',
  Sensor = 'speed'
}

export function getDeviceIcon(dev_type: DeviceType): string {
  return DeviceTypeIcon[dev_type]
}

export enum ConnectionStatus {
  Disconnected = 'Disconnected',
  Connected = 'Connected'
}

export enum DeviceStatus {
  Unsafe = 'Unsafe',
  Operational = 'Operational',
}

export class DeviceFields {
  field_name: string = ''
  field_value: string = ''
}

export class DeviceCommands {
  cmd_name: string = ''
  cmd_value: number = 0
}
// ***

// common device properties
export interface IDevice {
  id: string
  name: string
  device_type: DeviceType
  ip_address: string
  port: number
  connection_status: ConnectionStatus
  device_status: DeviceStatus
  fields: Array<DeviceFields>
  commands: Array<DeviceCommands>
}

// common device functions to manipulate device properties
export class Device {
  id: string = this.generateID()
  name: string = 'Device'
  device_type: DeviceType = DeviceType.Battery
  ip_address: string = '127.0.0.1'
  port: number = 0
  connection_status: ConnectionStatus = ConnectionStatus.Connected
  device_status: DeviceStatus = DeviceStatus.Operational
  fields: Array<DeviceFields> = Array<DeviceFields>(0)
  commands: Array<DeviceCommands> = Array<DeviceCommands>(0)

  clear() {
    this.id = this.generateID()
    this.name = 'Device'
    this.device_type = DeviceType.Battery
    this.ip_address = '127.0.0.1'
    this.port = 0
    this.connection_status = ConnectionStatus.Connected
    this.device_status = DeviceStatus.Operational
    this.fields = Array<DeviceFields>(0)
    this.commands= Array<DeviceCommands>(0)
  }

  clone(dev: Device) {
    this.id = dev.id
    this.name = dev.name
    this.device_type = dev.device_type
    this.ip_address = dev.ip_address
    this.port = dev.port
    this.connection_status = dev.connection_status
    this.device_status = dev.device_status
    this.fields = JSON.parse(JSON.stringify(dev.fields)) as DeviceFields[]
    this.commands = JSON.parse(JSON.stringify(dev.commands)) as DeviceCommands[]
  }

  generateID() { // TEMPORARY FUNCTION, MOVE TO RUST FRONTEND EVENTUALLY
    return Math.random().toString(36).substring(2, 9)
  }
}

// *** MOVE THESE TO CLIENT SIDE CONFIG FILE/DATABASE ENTRY TO ALLOW USER MODIFICATIONS OF DEFAULTS + ADDITIONAL DEVICE TYPES EVENTUALLY
// battery specific properties/functions
export class Battery extends Device implements IDevice {
  id = this.generateID()
  name = 'Battery 1'
  device_type = DeviceType.Battery
  ip_address = '127.0.0.1'
  port = 0
  connection_status = ConnectionStatus.Connected
  device_status = DeviceStatus.Operational
  fields = [{ field_name: 'Temperature', field_value: '' },
            { field_name: 'Power', field_value: '' }]
  commands = []
}

// inverter specific properties/functions
export class Inverter extends Device implements IDevice {
  id = this.generateID()
  name = 'Inverter 1'
  device_type = DeviceType.Inverter
  ip_address = '127.0.0.1'
  port = 0
  connection_status = ConnectionStatus.Connected
  device_status = DeviceStatus.Unsafe
  fields = [{ field_name: 'Inverter Field 1', field_value: '' },
            { field_name: 'Inverter Field 2', field_value: '' }]
  commands = []
}

// sensor specific properties/functions
export class Sensor extends Device implements IDevice {
  id = this.generateID()
  name = 'Sensor 1'
  device_type = DeviceType.Sensor
  ip_address = '127.0.0.1'
  port = 0
  connection_status = ConnectionStatus.Disconnected
  device_status = DeviceStatus.Unsafe
  fields = []
  commands = []
}
// ***
