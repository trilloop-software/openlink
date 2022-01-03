import { invoke } from '@tauri-apps/api/tauri'
import { IPv4 } from 'ip-num/IPNumber'

// *** custom types to use as device properties
export enum DeviceType {
  Battery = 'BATTERY',
  Inverter = 'INVERTER',
  Sensor = 'SENSOR'
}

export enum DeviceTypeIcon {
  Battery = 'battery_full',
  Inverter = 'bolt',
  Sensor = 'speed'
}

export enum ConnectionStatus {
  Disconnected,
  Connected
}

export enum DeviceStatus {
  Unsafe,
  Operational,
}

export class DeviceFields {
  field_name: string = ''
  field_value: string = ''
}
// ***

// common device properties
export interface Device {
  id: string
  name: string
  device_type: DeviceType
  icon: DeviceTypeIcon
  ip_address: IPv4
  port: Uint16Array
  connection_status: ConnectionStatus
  device_status: DeviceStatus
  fields: Array<DeviceFields>
}

// common device functions to manipulate device properties
export class DeviceFunctions {
  generateID() { // TEMPORARY FUNCTION, MOVE TO RUST FRONTEND EVENTUALLY
    return Math.random().toString(36).substring(2, 9)
  }
  ping(name, ip, port) {
    invoke('ping_device', { name: name })
      .then((response) => {
        alert('Successful: ' + response)
      })
      .catch((error) => {
        alert('Error: ' + error)
      })
  }
}

// *** MOVE THESE TO CLIENT SIDE CONFIG FILE/DATABASE ENTRY TO ALLOW USER MODIFICATIONS OF DEFAULTS + ADDITIONAL DEVICE TYPES EVENTUALLY
// battery specific properties/functions
export class Battery extends DeviceFunctions implements Device {
  id = this.generateID()
  name = 'Battery 1'
  device_type = DeviceType.Battery
  icon = DeviceTypeIcon.Battery
  ip_address = new IPv4('127.0.0.1')
  port = new Uint16Array(1)
  connection_status = ConnectionStatus.Connected
  device_status = DeviceStatus.Operational
  fields = [{ field_name: 'Temperature', field_value: '' },
            { field_name: 'Power', field_value: '' }]
}

// inverter specific properties/functions
export class Inverter extends DeviceFunctions implements Device {
  id = this.generateID()
  name = 'Inverter 1'
  device_type = DeviceType.Inverter
  icon = DeviceTypeIcon.Inverter
  ip_address = new IPv4('127.0.0.1')
  port = new Uint16Array(1)
  connection_status = ConnectionStatus.Connected
  device_status = DeviceStatus.Unsafe
  fields = [{ field_name: 'Inverter Field 1', field_value: '' },
            { field_name: 'Inverter Field 2', field_value: '' }]
}

// sensor specific properties/functions
export class Sensor extends DeviceFunctions implements Device {
  id = this.generateID()
  name = 'Sensor 1'
  device_type = DeviceType.Sensor
  icon = DeviceTypeIcon.Sensor
  ip_address = new IPv4('127.0.0.1')
  port = new Uint16Array(1)
  connection_status = ConnectionStatus.Disconnected
  device_status = DeviceStatus.Unsafe
  fields = []
}
// ***
