import Surreal from "https://deno.land/x/surrealdb@v0.2.0/mod.ts";
import { Request, Response } from "https://deno.land/x/oak@v11.1.0/mod.ts";

import { db_url, responseSkeleton } from "../helper.ts";

const login = async ({ request, response }: { request: Request; response: Response }) => {
	await responseSkeleton(response, async () => {
		const body = await request.body().value;
		const db = new Surreal(`${db_url}/rpc`, null);
		const token = await db.signin({
			NS: "global",
			DB: "global",
			SC: "global",
			username: body.username,
			pass: body.password,
		});

		return { token: token };
	});
};

const signup = async ({ request, response }: { request: Request; response: Response }) => {
	await responseSkeleton(response, async () => {
		const body = await request.body().value;
		const result = await Surreal.Instance.query(`SELECT * FROM user WHERE username = $username`, {
			username: body.username,
		});
		if (result[0].result.length > 0) {
			throw new Error(`Username already in use. Try ${request.url.origin}/login`);
		}
		const db = new Surreal(`${db_url}/rpc`, null);
		const token: string = await db.signup({
			NS: "global",
			DB: "global",
			SC: "global",
			username: body.username,
			pass: body.password,
		});
		db.close();

		return { token: token };
	});
};

export { login, signup };
