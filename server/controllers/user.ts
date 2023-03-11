import Surreal from "https://deno.land/x/surrealdb@v0.2.0/mod.ts";
import { Request, Response } from "https://deno.land/x/oak@v11.1.0/mod.ts";

import { decode_jwt, responseSkeleton } from '../helper.ts';

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
        return (await Surreal.Instance.select("user"));
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
        await Surreal.Instance.authenticate(jwt);
        const result = await Surreal.Instance.change(params.id, await request.body().value);
        await Surreal.Instance.invalidate();

        return result;
    })
}

const addPOI = async ({ params, request, response }: { params: {id: string}; request: Request; response: Response }) => {
    await responseSkeleton(response, async () => {
        const jwt = checkToken({params, request, ignoreIdMistmatch: true});
        
        await Surreal.Instance.authenticate(jwt);
        let result = await Surreal.Instance.select(params.id);
        if (result.length != 1) {
            throw new Error("User not found");
        }

        result = await Surreal.Instance.query("IF ($auth.id) != $other THEN (RELATE ($auth.id)->follows->$other) END", {
            other: params.id
        });
        await Surreal.Instance.invalidate();

        return result;
    });
}

const removePOI = async ({ params, request, response }: { params: {id: string}; request: Request; response: Response }) => {
    await responseSkeleton(response, async () => {
        const jwt = checkToken({params, request, ignoreIdMistmatch: true});
        
        await Surreal.Instance.authenticate(jwt);
        
        const result = await Surreal.Instance.query("DELETE follows WHERE in = $auth.id AND out = $remove", {
            remove: params.id
        });
        await Surreal.Instance.invalidate();

        return result;
    });
}

export {getUsers, getUser, editUser, addPOI, removePOI}