import Surreal from "https://deno.land/x/surrealdb@v0.2.0/mod.ts";
import { Request, Response } from "https://deno.land/x/oak@v11.1.0/mod.ts";
import { hash } from "https://deno.land/x/scrypt@v4.2.1/mod.ts";

import { User } from '../models/user.ts'

const responseSkeleton = async (response: Response, callBack: () => unknown) => {
    try {
        const result = await callBack();

        response.status = 200;
        response.body = {
            "message": "ok",
            "response": result
        }
    } catch (e) {
        response.status = 400;
        response.body = {
            "message": "malformed request",
            "error": {
                "name": e.name,
                "message": e.message
            }
        }
    }
}

const getUsers = async ({ response }: { response: Response }) => {
    await responseSkeleton(response, async () => {
        return (await Surreal.Instance.query(`SELECT * FROM user`))[0];
    });
}

const getUser = async ({ params, response }: { params: {id: string}; response: Response }) => {
    await responseSkeleton(response, async () => {
        return (await Surreal.Instance.query(`SELECT * FROM user WHERE id = ${params.id}`))[0];
    });
}

const editUser = async ({ params, request, response }: { params: {id: string}; request: Request; response: Response }) => {
    await responseSkeleton(response, async () => {
        const jwt: string|null = request.headers.get("X-Token");
        if (!jwt) {
            throw new Error("\"X-Token\" missing in header");
        }
        let body: User = await request.body().value;
        if (body.pass) {
            body.pass = hash(body.pass);
        }
        await Surreal.Instance.authenticate(jwt);
        body = await Surreal.Instance.change(params.id, body);
        await Surreal.Instance.invalidate();
        return body;
    })
}

const addPOI = async ({ params, request, response }: { params: {id: string}; request: Request; response: Response }) => {
    try {
        const jwt: string|null = request.headers.get("X-Token");
        if (!jwt) {
            throw new Error("X-Token missing in header");
        }
        const users: string[] = (await request.body().value).users;
        
        await Surreal.Instance.authenticate(jwt);
        let result = await Surreal.Instance.select(params.id);
        if (result.length != 1) {
            throw new Error("User not found")
        }
        const user: User = result[0];
        const following: string[] = user.following ?? [];
        const real_users: User[] = (await Surreal.Instance.query(`SELECT * FROM user WHERE id INSIDE [${users.join(",")}]`))[0].result;
        following.push(...real_users.flatMap(u => u.id ? [u.id] : []));
        user.following = [...new Set(following)];
        result = await Surreal.Instance.update(params.id, user);
        await Surreal.Instance.invalidate();

        response.status = 200;
        response.body = {
            "message": "ok",
            "user": result
        }
    } catch (e) {
        response.status = 400
        response.body = {
            "message": "malformed request",
            "error": {
                "name": e.name,
                "message": e.message
            }
        }
    }
}

const removePOI = async ({ params, request, response }: { params: {id: string}; request: Request; response: Response }) => {
    try {
        const jwt: string|null = request.headers.get("X-Token");
        if (!jwt) {
            throw new Error("X-Token missing in header");
        }
        const users: string[] = (await request.body().value).users;
        
        await Surreal.Instance.authenticate(jwt);
        let result = await Surreal.Instance.select(params.id);
        if (result.length != 1) {
            throw new Error("User not found")
        }
        const user: User = result[0];
        user.following = user.following ? user.following.filter(id => !users.includes(id)) : [];
        result = await Surreal.Instance.update(params.id, user);
        await Surreal.Instance.invalidate();

        response.status = 200;
        response.body = {
            "message": "ok",
            "user": result
        }
    } catch (e) {
        response.status = 400
        response.body = {
            "message": "malformed request",
            "error": {
                "name": e.name,
                "message": e.message
            }
        }
    }
}

const deleteUser = async ({ params, request, response }: { params: {id: string}; request: Request; response: Response }) => {
    try {
        const jwt: string|null = request.headers.get("X-Token");
        if (!jwt) {
            throw new Error("X-Token missing in header");
        }
        console.log(params.id);
        
        await Surreal.Instance.authenticate(jwt);
        const result = await Surreal.Instance.query(`SELECT * FROM user WHERE id = ${params.id}`);
        await Surreal.Instance.invalidate();

        response.status = 200;
        response.body = {
            "message": "ok",
            "user": result
        }
    } catch (e) {
        response.status = 400
        response.body = {
            "message": "malformed request",
            "error": {
                "name": e.name,
                "message": e.message
            }
        }
    }
}

export {getUsers, getUser, editUser, addPOI, removePOI, deleteUser}