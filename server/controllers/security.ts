import Surreal from 'https://deno.land/x/surrealdb@v0.2.0/mod.ts';
import { Request, Response, type BodyJson } from 'https://deno.land/x/oak@v10.6.0/mod.ts';

const login = async ({ request, response }: { request: Request; response: Response }) => {
    try {
        let body = await request.body().value;
        let token: String = await Surreal.Instance.signin({
            NS: 'global',
            DB: 'global',
            SC: 'global',
            username: body.username,
            pass: body.password
        });
        Surreal.Instance.invalidate();
    
        response.status = 200;
        response.body = {
            "token": token
        }
    } catch (e) {
        response.status = 400
        response.body = {
            "message": "malformed request",
            "error": e
        }
    }
}

const signup = async ({ request, response }: { request: Request; response: Response }) => {
    try {
        let body = await request.body().value;
        let result = await Surreal.Instance.query("SELECT * FROM user WHERE username = $username", {username: body.username});
        if (result[0].result.length > 0) {
            throw new Error(`Username already in use. Try ${request.url.origin}/login`);
        }
        let token: String = await Surreal.Instance.signup({
            NS: 'global',
            DB: 'global',
            SC: 'global',
            username: body.username,
            pass: body.password
        });
        Surreal.Instance.invalidate();
    
        response.status = 200;
        response.body = {
            "token": token
        }
    } catch (e) {
        console.log(e);
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

export {login, signup}