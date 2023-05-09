import Surreal from "https://deno.land/x/surrealdb@v0.2.0/mod.ts";
import { Request, Response } from "https://deno.land/x/oak@v11.1.0/mod.ts";

import { db_url, decode_jwt, responseSkeleton } from "../helper.ts";

const createPost = async ({ request, response }: { request: Request; response: Response }) => {
	await responseSkeleton(response, async () => {
		const body: { message: string | null } = await request.body().value;
		if (!body.message) {
			throw new Error("post message must be specified");
		}

		const db = new Surreal(`${db_url}/rpc`, request.headers.get("X-Token"));
		await db.use("global", "global");

		if (
			request.url.searchParams.has("parent") &&
			!(
				await Surreal.Instance.query("SELECT * FROM $post", {
					post: request.url.searchParams.get("parent"),
				})
			)[0].result[0]
		) {
			throw new Error("specified parent post doesn't exist");
		}

		const result = await db.query("CREATE post SET message = $message", {
			message: body.message,
		});
		if (request.url.searchParams.has("parent") && result[0].status === "OK") {
			await db.query("RELATE ($parent)->response->($new)", {
				parent: request.url.searchParams.get("parent"),
				new: result[0].result[0].id,
			});
		}

		db.close();

		return result;
	});
};

const getPosts = async ({
	params,
	request,
	response,
}: {
	params?: { id?: string };
	request: Request;
	response: Response;
}) => {
	await responseSkeleton(response, async () => {
		//sleep for 1 second
		await new Promise((resolve) => setTimeout(resolve, 1000));

		const fields = [
			"*",
			"count(<-likes<-user) AS likes",
			"count(<-dislikes<-user) AS dislikes",
			"count(->response->post) AS responses",
		];
		if (request.url.searchParams.has("as")) {
			fields.push("$user INSIDE <-likes<-user AS liked");
			fields.push("$user INSIDE <-dislikes<-user AS disliked");
		}

		const conditions: string[] = [];
		if (request.url.searchParams.has("after")) conditions.push("time > $after");
		if (params?.id) conditions.push("id = $id");
		if (request.url.searchParams.has("parent")) conditions.push("$parent INSIDE <-response<-post");
		else if (!params?.id) conditions.push("count(<-response<-post) = 0");

		return (
			await Surreal.Instance.query(
				`SELECT ${fields.join(",")} FROM post WHERE ${conditions.join(
					" AND "
				)} ORDER BY time LIMIT 25 FETCH author`,
				{
					parent: request.url.searchParams.get("parent") ?? "",
					after: request.url.searchParams.get("after") ?? "",
					user: request.url.searchParams.get("as") ?? "",
					id: params?.id ?? "",
				}
			)
		)[0];
	});
};

const editPost = async ({
	params,
	request,
	response,
}: {
	params: { id: string };
	request: Request;
	response: Response;
}) => {
	await responseSkeleton(response, async () => {
		const body: { message: string | null } = await request.body().value;
		if (!body.message) {
			throw new Error("post message must be specified");
		}

		const jwt = request.headers.get("X-Token");
		if (!jwt) {
			throw new Error("cannot edit post without authentication");
		}

		const json = decode_jwt(jwt);
		const post: { id: string; author: string } = (
			await Surreal.Instance.query("SELECT id, author FROM $id", {
				id: params.id,
			})
		)[0].result[0];

		if (json.payload.ID != post.author) {
			throw new Error("unable to edit post not made by you");
		}

		const db = new Surreal(`${db_url}/rpc`, jwt);
		const result = await db.change(post.id, { message: body.message });
		db.close();

		return result;
	});
};

const deletePost = async ({
	params,
	request,
	response,
}: {
	params: { id: string };
	request: Request;
	response: Response;
}) => {
	await responseSkeleton(response, async () => {
		const jwt = request.headers.get("X-Token");
		if (!jwt) {
			throw new Error("cannot delete post without authentication");
		}

		const json = decode_jwt(jwt);
		const post = (
			await Surreal.Instance.query("SELECT id, author FROM $id", {
				id: params.id,
			})
		)[0].result[0];
		if (!post) {
			throw new Error("post doesn't exist");
		}

		const account = (
			await Surreal.Instance.query("SELECT admin FROM $id", {
				id: json.payload.ID,
			})
		)[0].result[0];

		if (!account.admin && (!post.author_id || json.payload.ID != post.author_id)) {
			throw new Error("unable to delete post not made by you");
		}

		const db = new Surreal(`${db_url}/rpc`, jwt);
		const result = await db.delete(post.id);
		db.close();

		return result;
	});
};

const relateOnTable = async ({
	params,
	request,
	response,
	table,
}: {
	params: { id: string };
	request: Request;
	response: Response;
	table: "likes" | "dislikes";
}) => {
	await responseSkeleton(response, async () => {
		const jwt = request.headers.get("X-Token");
		if (!jwt) {
			throw new Error("cannot like post without authentication");
		}

		const post = (await Surreal.Instance.query(`SELECT id, author_id FROM ${params.id}`))[0]
			.result[0];
		if (!post) {
			throw new Error("post doesn't exist");
		}

		const json = decode_jwt(jwt);
		const account: { id: string } = (
			await Surreal.Instance.query(`SELECT id FROM ${json.payload.ID}`)
		)[0].result[0];
		if (!account) {
			throw new Error("unable to (dis)like post with invalid user id");
		}

		const db = new Surreal(`${db_url}/rpc`, jwt);
		if (!request.url.searchParams.has("reset")) {
			await db.query(
				`DELETE ${table == "likes" ? "dislikes" : "likes"} WHERE in = ($auth.id) AND out = $post`,
				{ post: post.id }
			);
		}
		await db.query(
			request.url.searchParams.has("reset")
				? `DELETE ${table} WHERE in = ($auth.id) AND out = $post`
				: `RELATE ($auth.id)->${table}->($post)`,
			{
				post: post.id,
			}
		);

		const result = (
			await db.query(
				`SELECT *, count(<-likes<-user) AS likes, count(<-dislikes<-user) AS dislikes, count(->response->post) AS responses, ($auth.id) INSIDE <-dislikes<-user AS disliked, ($auth.id) INSIDE <-likes<-user AS liked FROM $post FETCH author`,
				{
					post: post.id,
				}
			)
		)[0];

		db.close();

		return result;
	});
};

const likePost = async ({
	params,
	request,
	response,
}: {
	params: { id: string };
	request: Request;
	response: Response;
}) => {
	await relateOnTable({ params, request, response, table: "likes" });
};

const dislikePost = async ({
	params,
	request,
	response,
}: {
	params: { id: string };
	request: Request;
	response: Response;
}) => {
	await relateOnTable({ params, request, response, table: "dislikes" });
};

export { createPost, getPosts, editPost, deletePost, likePost, dislikePost };
