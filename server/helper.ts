import { Response } from "https://deno.land/x/oak@v11.1.0/mod.ts";

const db_url = "http://127.0.0.1:8000";

const decode_jwt = (jwt: string) => {
	const [header, payload] = jwt
		.split(".")
		.slice(0, 2)
		.map((val: string) => {
			const json: string = atob(val);
			return JSON.parse(json);
		});
	return { header: header, payload: payload };
};

/**
 * provide a simplified way to handle errors and set the response status
 *
 * @param response: the original response
 * @param callBack: the function to be executed to handle the request
 */
const responseSkeleton = async (response: Response, callBack: () => unknown) => {
	try {
		const result = await callBack();

		response.status = 200;
		response.body = {
			message: "ok",
			content: result,
		};
	} catch (e) {
		response.status = 400;
		response.body = {
			message: "malformed request",
			error: {
				name: e.name,
				message: e.message,
			},
		};
	}
};

export { db_url, decode_jwt, responseSkeleton };
