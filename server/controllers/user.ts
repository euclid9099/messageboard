import Surreal from "https://deno.land/x/surrealdb@v0.2.0/mod.ts";
import { Request, Response } from "https://deno.land/x/oak@v11.1.0/mod.ts";

import { db_url, decode_jwt, responseSkeleton } from '../helper.ts';

const checkToken = ({ params, request, ignoreIdMistmatch }: { params: {id: string}; request: Request; ignoreIdMistmatch?: boolean }) => {
    const jwt: string|null = request.headers.get("X-Token");
    if (!jwt) {
        throw new Error("\"X-Token\" missing in header");
    }
    const json = decode_jwt(jwt);
    if (!ignoreIdMistmatch && json.payload.ID != params.id) {
        throw new Error("can only edit self");      
    }
    return jwt;
}

const getUsers = async ({ response }: { response: Response }) => {
    await responseSkeleton(response, async () => {
        return await Surreal.Instance.query("SELECT *, count(->follows->user.id) AS follows, count(<-follows<-user.id) AS followers FROM user ORDER BY followers DESC");
    });
}

const getUser = async ({ params, response }: { params: {id: string}; response: Response }) => {
    await responseSkeleton(response, async () => {
        return await Surreal.Instance.query("SELECT *, ->follows->user.id AS follows, <-follows<-user.id AS followers FROM user WHERE id = $request_id", {
            "request_id": params.id
        });
    });
}

const editUser = async ({ params, request, response }: { params: {id: string}; request: Request; response: Response }) => {
    await responseSkeleton(response, async () => {
        //needs to check if token exists and matches the user we want to edit
        const jwt = checkToken({params, request, ignoreIdMistmatch: true});

        //only oneself can update so we need to authenticate and invalidate afterwards
        const db = new Surreal(`${db_url}/rpc`, jwt);
        const result = await db.change(params.id, await request.body().value);
        db.close();

        return result;
    })
}

const followUser = async ({ params, request, response }: { params: {id: string}; request: Request; response: Response }) => {
    await responseSkeleton(response, async () => {
        const jwt = checkToken({params, request, ignoreIdMistmatch: true});
        
        let result = await Surreal.Instance.select(params.id);
        if (result.length != 1) {
            throw new Error("User not found");
        }

        const db = new Surreal(`${db_url}/rpc`, jwt);
        result = await db.query("IF ($auth.id) != $other THEN (RELATE ($auth.id)->follows->$other) END", {
            other: params.id
        });
        db.close();

        return result;
    });
}

const unfollowUser = async ({ params, request, response }: { params: {id: string}; request: Request; response: Response }) => {
    await responseSkeleton(response, async () => {
        const jwt = checkToken({params, request, ignoreIdMistmatch: true});
        
        const db = new Surreal(`${db_url}/rpc`, jwt);
        
        const result = await db.query("DELETE follows WHERE in = $auth.id AND out = $remove", {
            remove: params.id
        });
        db.close();

        return result;
    });
}

export {getUsers, getUser, editUser, followUser, unfollowUser}