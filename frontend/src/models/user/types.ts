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

export interface getUser {
    code: string,
    description: User
}

export interface getUserList {
    code: string,
    description: User[]
}
