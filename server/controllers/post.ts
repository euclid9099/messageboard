import Surreal from "https://deno.land/x/surrealdb@v0.2.0/mod.ts";
import { Request, Response } from "https://deno.land/x/oak@v11.1.0/mod.ts";

import { decode_jwt, responseSkeleton } from '../helper.ts';

const createPost = async ({ request, response }: { request: Request, response: Response }) => {
    await responseSkeleton(response, async () => {
        const body: {message: string|null} = await request.body().value;
        if (!body.message) {
            throw new Error("post message must be specified");
        }

        const jwt = request.headers.get("X-Token");
        if (jwt) {
            await Surreal.Instance.authenticate(jwt);
        }

        const response = await Surreal.Instance.query(`CREATE post SET message = \"${body.message}\"`);
        const post: null|{id: string|null} = response[0].result[0];
        if (post && post.id && jwt) {
            const author_id = decode_jwt(jwt).payload.ID;
            if (author_id) {
                await Surreal.Instance.query(`RELATE ${author_id}->posted->${post.id}`);
            }
        }
        await Surreal.Instance.invalidate();

        return response;
    });
}

const getPosts = async ({ response }: { response: Response }) => {
    await responseSkeleton(response, async () => {
        return await Surreal.Instance.query("SELECT *, <-posted<-user.id AS author FROM post");
    });
}

const getPost = async ({ params, response }: { params: {id: string}; response: Response }) => {
    await responseSkeleton(response, async () => {
        return await Surreal.Instance.query(`SELECT * FROM post WHERE id = ${params.id}`);
    });
}

const editPost = async ({ params, request, response }: { params: {id: string}; request: Request; response: Response }) => {
    await responseSkeleton(response, async () => {
        const body: {message: string|null} = await request.body().value;
        if (!body.message) {
            throw new Error("post message must be specified");
        }

        const jwt = request.headers.get("X-Token");
        if (!jwt) {
            throw new Error("cannot edit post without authentication");
        }
        
        const json = decode_jwt(jwt);
        const post: {id: string, author_id: string} = (await Surreal.Instance.query(`SELECT id, author_id FROM ${params.id}`))[0].result[0];

        if (json.payload.ID != post.author_id) {
            throw new Error("unable to edit post not made by you");
        }

        await Surreal.Instance.authenticate(jwt);
        const result = await Surreal.Instance.change(post.id, {message: body.message, edited: true});
        await Surreal.Instance.invalidate();

        return result;
    });
}

const deletePost = async ({ params, request, response }: { params: {id: string}; request: Request; response: Response }) => {
    await responseSkeleton(response, async () => {
        const jwt = request.headers.get("X-Token");
        if (!jwt) {
            throw new Error("cannot delete post without authentication");
        }
        
        const json = decode_jwt(jwt);
        const post = (await Surreal.Instance.query(`SELECT id, author_id FROM ${params.id}`))[0].result[0];
        if (!post) {
            throw new Error("post doesn't exist");
        }

        const account = (await Surreal.Instance.query(`SELECT admin FROM ${json.payload.ID}`))[0].result[0];
        
        if (!account.admin && (!post.author_id || json.payload.ID != post.author_id)) {
            throw new Error("unable to delete post not made by you");
        }

        await Surreal.Instance.authenticate(jwt);
        const result = await Surreal.Instance.delete(post.id);
        await Surreal.Instance.invalidate();

        return result;
    });
}

const likePost = async ({ params, request, response }: { params: {id: string }; request: Request; response: Response }) => {
    await responseSkeleton(response, async () => {
        const jwt = request.headers.get("X-Token");
        if (!jwt) {
            throw new Error("cannot like post without authentication");
        }
        
        const json = decode_jwt(jwt);
        const post = (await Surreal.Instance.query(`SELECT id, author_id FROM ${params.id}`))[0].result[0];
        if (!post) {
            throw new Error("post doesn't exist");
        }

        const account: {id: string} = (await Surreal.Instance.query(`SELECT id FROM ${json.payload.ID}`))[0].result[0];
        if (!account) {
            throw new Error("unable to like post with invalid user id");
        }

        await Surreal.Instance.authenticate(jwt);
        let result;
        if (request.url.searchParams.has("reset")) {
            result = await Surreal.Instance.query(`UPDATE ${params.id} SET likes -= ${account.id}`);
        } else {
            result = await Surreal.Instance.query(`UPDATE ${params.id} SET likes += ${account.id}, dislikes -= ${account.id}`);
        }
        await Surreal.Instance.invalidate();

        return result;
    });
}

const dislikePost = async ({ params, request, response }: { params: {id: string }; request: Request; response: Response }) => {
    await responseSkeleton(response, async () => {
        const jwt = request.headers.get("X-Token");
        if (!jwt) {
            throw new Error("cannot like post without authentication");
        }
        
        const json = decode_jwt(jwt);
        const post = (await Surreal.Instance.query(`SELECT id, author_id FROM ${params.id}`))[0].result[0];
        if (!post) {
            throw new Error("post doesn't exist");
        }

        const account: {id: string} = (await Surreal.Instance.query(`SELECT id FROM ${json.payload.ID}`))[0].result[0];
        if (!account) {
            throw new Error("unable to like post with invalid user id");
        }

        await Surreal.Instance.authenticate(jwt);
        let result;
        if (request.url.searchParams.has("reset")) {
            result = await Surreal.Instance.query(`UPDATE ${params.id} SET dislikes -= ${account.id}`);
        } else {
            result = await Surreal.Instance.query(`UPDATE ${params.id} SET dislikes += ${account.id}, likes -= ${account.id}`);
        }
        await Surreal.Instance.invalidate();

        return result;
    });
}

const createResponse = async ({ params, request, response }: { params: {id: string}; request: Request, response: Response }) => {
    await responseSkeleton(response, async () => {
        const post: {message: string|null} = await request.body().value;
        if (!post.message) {
            throw new Error("post message must be specified");
        }

        const jwt = request.headers.get("X-Token");
        if (jwt) {
            await Surreal.Instance.authenticate(jwt);
        }

        const parent = (await Surreal.Instance.query(`SELECT id FROM ${params.id}`))[0].result[0];
        if (!parent) {
            throw new Error("post doesn't exist");
        }

        const result = await Surreal.Instance.query(`CREATE post SET message = \"${post.message}\", parent = ${parent.id}`);
        await Surreal.Instance.invalidate();

        return result;
    });
}

const getResponses = async ({ params, response }: { params: {id: string}; response: Response }) => {
    await responseSkeleton(response, async () => {
        return await Surreal.Instance.query(`SELECT id, author_id, message, parent, original_parent, edited, count(likes) AS amount_likes, count(dislikes) AS amount_dislikes FROM post WHERE parent = ${params.id}`);
    });
}

export {createPost, getPosts, getPost, editPost, deletePost, likePost, dislikePost, createResponse, getResponses}