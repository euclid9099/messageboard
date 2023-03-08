interface User {
    id: string|null,
    pass: string|null,
    following: string[]|null,
    admin: boolean|null,
    username: string|null
}

export type {User}