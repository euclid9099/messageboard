import Surreal from "https://deno.land/x/surrealdb@v0.2.0/mod.ts";
import { Request, Response } from "https://deno.land/x/oak@v11.1.0/mod.ts";
import { hash } from "https://deno.land/x/scrypt@v4.2.1/mod.ts";

import { User } from '../models/user.ts'

import { decode_jwt, responseSkeleton } from '../helper.ts';

const checkToken = ({ params, request }: { params: {id: string}; request: Request; }) => {
    const jwt: string|null = request.headers.get("X-Token");
    if (!jwt) {
        throw new Error("\"X-Token\" missing in header");
    }
    const json = decode_jwt(jwt);
    if (json.payload.ID != params.id) {
        throw new Error("can only edit self");      
    }
    return jwt;
}

const getUsers = async ({ response }: { response: Response }) => {
    await responseSkeleton(response, async () => {
        return (await Surreal.Instance.select("user"));
    });
}

const getUser = async ({ params, response }: { params: {id: string}; response: Response }) => {
    await responseSkeleton(response, async () => {
        return await Surreal.Instance.select(params.id);
    });
}

const editUser = async ({ params, request, response }: { params: {id: string}; request: Request; response: Response }) => {
    await responseSkeleton(response, async () => {
        //needs to check if token exists and matches the user we want to edit
        const jwt = checkToken({params, request});

        let body: User = await request.body().value;
        if (body.pass) {
            body.pass = hash(body.pass);
        }

        //only oneself can update so we need to authenticate and invalidate afterwards
        await Surreal.Instance.authenticate(jwt);
        body = await Surreal.Instance.change(params.id, body);
        await Surreal.Instance.invalidate();

        return body;
    })
}

const addPOI = async ({ params, request, response }: { params: {id: string}; request: Request; response: Response }) => {
    await responseSkeleton(response, async () => {
        const jwt = checkToken({params, request});
        
        await Surreal.Instance.authenticate(jwt);
        let result = await Surreal.Instance.select(params.id);
        if (result.length != 1) {
            throw new Error("User not found");
        }

        const user: User = result[0];
        const following: string[] = user.following ?? [];

        //filter for users which actually exist
        const users: string[] = (await request.body().value).users;
        const real_users: User[] = (await Surreal.Instance.query(`SELECT id FROM user WHERE id INSIDE [${users.join(",")}]`))[0].result;
        following.push(...real_users.flatMap(u => u.id ? [u.id] : []));
        user.following = [...new Set(following)];

        result = await Surreal.Instance.update(params.id, user);
        await Surreal.Instance.invalidate();

        return result;
    });
}

const removePOI = async ({ params, request, response }: { params: {id: string}; request: Request; response: Response }) => {
    await responseSkeleton(response, async () => {
        const jwt = checkToken({params, request});
        
        await Surreal.Instance.authenticate(jwt);
        let result = await Surreal.Instance.select(params.id);
        if (result.length != 1) {
            throw new Error("User not found")
        }
        const user: User = result[0];
        
        const to_remove: string[] = (await request.body().value).users;
        user.following = user.following ? user.following.filter(id => !to_remove.includes(id)) : [];
        result = await Surreal.Instance.update(params.id, user);
        await Surreal.Instance.invalidate();

        return result;
    });
}

export {getUsers, getUser, editUser, addPOI, removePOI}