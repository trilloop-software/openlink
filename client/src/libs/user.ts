export enum UserEditMode {
  Unselected,
  Password,
  Usergroup
}

export interface IUser {
  name: String,
  pwd: String,
  ugroup: number,
}

export class User implements IUser {
  name = ""
  pwd = ""
  ugroup = undefined

  clear() {
    this.name = ""
    this.pwd = ""
    this.ugroup = undefined
  }

  clone(user: User) {
    this.name = user.name
    this.pwd = user.pwd
    this.ugroup = user.ugroup
  }

  fromSecure(user: UserSecure) {
    this.name = user.name
    this.ugroup = user.ugroup
  }
}

export interface IUserSecure {
  name: String,
  ugroup: number,
}

export class UserSecure implements IUserSecure {
  name = ""
  ugroup = 1

  clear() {
    this.name = ""
    this.ugroup = 1
  }

  clone(user: UserSecure | User) {
    this.name = user.name
    this.ugroup = user.ugroup
  }
}

export function getUserGroup(ugroup: number) {
  switch(ugroup) {
    case 255:
      return 'Admin'
    case 2:
      return 'Software Team'
    case 1:
      return 'Mission Control'
    default:
      return 'Unknown'
  }
}
