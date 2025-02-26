
export interface UserVector {
  users: []
}

export interface wasmResponse {
  code: string,
  description: string
}
export interface LoginResponse {
  code: string,
  description: {
    auth: string,
    refresh: string
  },
}


export interface User {
  id: string,
  name: string,
  email: string,
  phone: string,
  institution: string,
  is_partner: boolean,
  first_time: boolean,
  role: string,

}
